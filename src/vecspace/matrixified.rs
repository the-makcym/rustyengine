use {
    std::{
        ops::{Index, IndexMut,
              Neg, Add, Sub, Mul, Div,
        },
    },
    crate::vecspace::{
        utils::{
            pow_minus, Pair, Num,
        },
        enums::{
            MatrixifiedError::{self, *},
            Sign,
            Ops,
        },
    },
};

// <<< Matrixified

pub trait Matrixified {
    type Elem;

    fn zeros(_: Pair) -> Self;
    fn fill_with(_: Pair, _: Self::Elem) -> Self where Self::Elem: Num;
    fn size(&self) -> Pair;
    fn transpose(&mut self);
    fn elem(&self, _: (usize, usize)) -> &Self::Elem;
    fn elem_mut(&mut self, _: (usize, usize)) -> &mut Self::Elem;
    fn norm(&self) -> f64;
    fn as_vector(self) -> Result<Vector<Self::Elem>, MatrixifiedError>;

    fn allow_add(&self, rhs: &impl Matrixified) -> Result<(), MatrixifiedError> {
        match self.size() == rhs.size() {
            true => Ok(()),
            false => Err(InappropriateSizes),
        }
    }

    fn allow_mul(&self, rhs: &impl Matrixified) -> Result<(), MatrixifiedError> {
        match self.size().x == rhs.size().y {
            true => Ok(()),
            false => Err(InappropriateSizes),
        }
    }

    fn m_add(&self, rhs: &impl Matrixified<Elem=Self::Elem>, sign: Sign)
             -> Result<Matrix<Self::Elem>, MatrixifiedError>
        where Self::Elem: Num
    {
        self.allow_add(rhs)?;

        let mut output = Matrix::zeros(self.size());
        for row in 0..self.size().y {
            for col in 0..self.size().x {
                output[(row, col)] = match sign {
                    Sign::Plus => *self.elem((row, col)) + *rhs.elem((row, col)),
                    Sign::Minus => *self.elem((row, col)) - *rhs.elem((row, col)),
                };
            }
        }
        Ok(output)
    }


    fn m_mul(&self, rhs: &impl Matrixified<Elem=Self::Elem>)
             -> Result<Matrix<Self::Elem>, MatrixifiedError>
        where Self::Elem: Num
    {
        self.allow_mul(rhs)?;
        let output_size = (self.size().y, rhs.size().x).into();
        let mut output = Matrix::zeros(output_size);

        for row in 0..output_size.y {
            for col in 0..output_size.x {
                output[(row, col)] = (0..self.size().x).map(|i| *self.elem((row, i)) * *rhs.elem((i, col))).sum();
            }
        }
        Ok(output)
    }

    fn a(&mut self, num: Self::Elem) where Self::Elem: Num {
        for r in 0..self.size().y {
            for c in 0..self.size().x {
                *self.elem_mut((r, c)) += num;
            }
        }
    }

    fn s(&mut self, num: Self::Elem) where Self::Elem: Num {
        for r in 0..self.size().y {
            for c in 0..self.size().x {
                *self.elem_mut((r, c)) -= num;
            }
        }
    }

    fn m(&mut self, num: Self::Elem) where Self::Elem: Num {
        for r in 0..self.size().y {
            for c in 0..self.size().x {
                *self.elem_mut((r, c)) *= num;
            }
        }
    }

    fn d(&mut self, num: Self::Elem) where Self::Elem: Num {
        for r in 0..self.size().y {
            for c in 0..self.size().x {
                *self.elem_mut((r, c)) /= num;
            }
        }
    }
}

// Matrixified >>>


// <<< Matrix

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    inner: Vec<Vec<T>>,
    transposed: bool,
    pub determinant: Option<T>,
    initial_size: Pair,
    pub actual_size: Pair,
}

impl<T: Num> Matrix<T> {
    pub fn identity(initial_size: Pair) -> Result<Self, MatrixifiedError> {
        if initial_size.x != initial_size.y {
            return Err(NonSquareMatrix);
        }
        let mut inner = vec![vec![T::zero(); initial_size.x]; initial_size.y];
        for d in 0..initial_size.y {
            inner[d][d] = T::one();
        }
        Ok(Self {
            inner,
            transposed: false,
            determinant: Some(T::one()),
            initial_size,
            actual_size: initial_size,
        })
    }

    pub fn is_transposed(&self) -> bool {
        return self.transposed;
    }

    pub fn comp_determ(&mut self) -> Result<(), MatrixifiedError> {
        if self.initial_size.x != self.initial_size.y {
            return Err(NonSquareMatrix);
        }

        if self.determinant.is_none() {
            let mut rows = vec![true; self.initial_size.y];
            let mut cols = vec![true; self.initial_size.x];
            self.determinant = Some(self.minor(&mut rows, &mut cols));
        }

        Ok(())
    }

    // computes inversed matrix for not-transposed matrix and then transposes it
    pub fn inverse(&self) -> Result<Self, MatrixifiedError> {
        if self.initial_size.x != self.initial_size.y {
            return Err(NonSquareMatrix);
        }
        let det;
        if let Some(d) = self.determinant {
            det = d;
        } else {
            return Err(UnknownDeterminant);
        }
        if det == T::zero() {
            return Err(ZeroDeterminant);
        }

        let mut rows = vec![true; self.initial_size.y];
        let mut cols = vec![true; self.initial_size.x];

        let mut inversed = Matrix::<T>::zeros(self.initial_size);
        for row in 0..self.initial_size.y {
            cols[row] = false;
            for col in 0..self.initial_size.x {
                rows[col] = false;
                inversed.inner[row][col] =
                    pow_minus::<T>(row + col) * self.minor(&mut rows, &mut cols) / det;
                rows[col] = true;
            }
            cols[row] = true;
        }

        if self.transposed {
            inversed.transpose();
        }

        Ok(inversed)
    }

    // does not pay any attention on whether the matrix is transposed or not
    fn minor(&self, rows: &mut Vec<bool>, cols: &mut Vec<bool>) -> T {
        // just for ensurance
        assert_eq!(self.initial_size.y, self.initial_size.x);

        // when this code is reached, matrix surely is square
        let mut row = 0;
        while row < self.initial_size.y && rows[row] == false { row += 1 }

        if row == self.initial_size.y {
            return T::one();
        } else {
            rows[row] = false;
        }

        let mut minor = T::zero();
        for col in 0..self.initial_size.x {
            let mut j = 0;
            if cols[col] {
                cols[col] = false;
                minor += pow_minus::<T>(j) * self.inner[row][col] * self.minor(rows, cols);
                cols[col] = true;
            }
        }
        rows[row] = true;

        minor
    }
}

impl<T: Num> Matrixified for Matrix<T> {
    type Elem = T;

    fn zeros(initial_size: Pair) -> Self {
        Self {
            inner: vec![vec![T::zero(); initial_size.x]; initial_size.y],
            transposed: false,
            determinant: None,
            initial_size,
            actual_size: initial_size,
        }
    }

    fn fill_with(initial_size: Pair, with: T) -> Self {
        Self {
            inner: vec![vec![with; initial_size.x]; initial_size.y],
            transposed: false,
            determinant: None,
            initial_size,
            actual_size: initial_size,
        }
    }

    fn size(&self) -> Pair {
        self.actual_size
    }

    fn transpose(&mut self) {
        self.transposed = !self.transposed;
        self.actual_size.transpose();
    }

    // use only after checking whether (r, c) is valid
    fn elem(&self, (r, c): (usize, usize)) -> &Self::Elem {
        assert!(Pair::from((r, c)).in_rect(self.size()));

        match self.transposed {
            false => &self.inner[r][c],
            true => &self.inner[c][r],
        }
    }

    // use only after checking whether (r, c) is valid
    fn elem_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Elem {
        assert!(Pair::from((r, c)).in_rect(self.size()));

        match self.transposed {
            false => &mut self.inner[r][c],
            true => &mut self.inner[c][r],
        }
    }

    fn norm(&self) -> f64 {
        self.inner
            .iter()
            .map(|row| row.iter()
                .map(|elem| (*elem * *elem).into())
                .sum::<f64>())
            .sum::<f64>()
            .sqrt()
    }

    fn as_vector(mut self) -> Result<Vector<T>, MatrixifiedError> {
        if self.actual_size.y != 1 && self.actual_size.x != 1 {
            return Err(NotAVector);
        }
        if self.actual_size.is_vertical() {
            self.transpose();
        }

        let mut output = Vector::zeros(self.actual_size);
        for col in 0..self.actual_size.x {
            output[(1, col)] = self[(1, col)];
        }
        Ok(output)
    }
}


// unary operators

impl<T: Num> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.elem(index)
    }
}

impl<T: Num> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        self.elem_mut(index)
    }
}

impl<T: Num> Neg for Matrix<T> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.m(-T::one());
        self
    }
}


// binary operators

// Matrix + [Matrix | Vector] = Matrix
impl<T: Num, M: Matrixified<Elem=T>> Add<&M> for &Matrix<T> {
    type Output = Result<Matrix<T>, MatrixifiedError>;

    fn add(self, rhs: &M) -> Self::Output {
        self.m_add(rhs, Sign::Plus)
    }
}

// Matrix - [Matrix | Vector] = Matrix
impl<T: Num, M: Matrixified<Elem=T>> Sub<&M> for &Matrix<T> {
    type Output = Result<Matrix<T>, MatrixifiedError>;

    fn sub(self, rhs: &M) -> Self::Output {
        self.m_add(rhs, Sign::Minus)
    }
}

// Matrix * [Matrix | Vector] = Matrix
impl<T: Num, M: Matrixified<Elem=T>> Mul<&M> for &Matrix<T> {
    type Output = Result<Matrix<T>, MatrixifiedError>;

    fn mul(self, rhs: &M) -> Self::Output {
        self.m_mul(rhs)
    }
}

// Matrix / Matrix = Matrix
impl<T: Num> Div for &Matrix<T> {
    type Output = Result<Matrix<T>, MatrixifiedError>;

    fn div(self, rhs: Self) -> Self::Output {
        self.mul(&(rhs.inverse()?))
    }
}

// Matrix >>>


// <<< Vector

#[derive(Debug, Clone)]
pub struct Vector<T> {
    inner: Vec<T>,
    transposed: bool,
    length: usize,
    actual_size: Pair,
}

impl<T: Num> Vector<T> {
    pub fn is_transposed(&self) {
        self.is_transposed()
    }
}

impl<T: Num> Matrixified for Vector<T> {
    type Elem = T;

    // size should looks like Pair { x: length, y: 1 }
    fn zeros(initial_size: Pair) -> Self {
        assert_eq!(initial_size.y, 1);

        Self {
            inner: vec![T::zero(); initial_size.x],
            transposed: false,
            length: initial_size.x,
            actual_size: initial_size,
        }
    }

    // size should looks like Pair { x: length, y: 1 }
    fn fill_with(initial_size: Pair, with: T) -> Self {
        assert_eq!(initial_size.y, 1);

        Self {
            inner: vec![with; initial_size.x],
            transposed: false,
            length: initial_size.x,
            actual_size: initial_size,
        }
    }

    fn size(&self) -> Pair {
        self.actual_size
    }

    fn transpose(&mut self) {
        self.transposed = !self.transposed;
        self.actual_size.transpose();
    }

    // use only after checking whether (r, c) is valid
    fn elem(&self, (r, c): (usize, usize)) -> &Self::Elem {
        assert!(Pair::from((r, c)).in_rect(self.size()));

        match self.transposed {
            false => &self.inner[c],
            true => &self.inner[r],
        }
    }

    // use only after checking whether (r, c) is valid
    fn elem_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Elem {
        assert!(Pair::from((r, c)).in_rect(self.size()));

        match self.transposed {
            false => &mut self.inner[c],
            true => &mut self.inner[r],
        }
    }

    fn norm(&self) -> f64 {
        self.inner
            .iter()
            .map(|elem| (*elem * *elem).into())
            .sum::<f64>()
            .sqrt()
    }

    fn as_vector(self) -> Result<Vector<T>, MatrixifiedError> {
        Ok(self)
    }
}


// unary operators

impl<T: Num> Index<(usize, usize)> for Vector<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.elem(index)
    }
}

impl<T: Num> IndexMut<(usize, usize)> for Vector<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        self.elem_mut(index)
    }
}

impl<T: Num> Neg for Vector<T> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.m(-T::one());
        self
    }
}


// binary operators

// Vector + [Matrix | Vector] = Vector
impl<T: Num, M: Matrixified<Elem=T>> Add<&M> for &Vector<T> {
    type Output = Result<Vector<T>, MatrixifiedError>;

    fn add(self, rhs: &M) -> Self::Output {
        self.m_add(rhs, Sign::Plus)?.as_vector()
    }
}

// Vector - [Matrix | Vector] = Vector
impl<T: Num, M: Matrixified<Elem=T>> Sub<&M> for &Vector<T> {
    type Output = Result<Vector<T>, MatrixifiedError>;

    fn sub(self, rhs: &M) -> Self::Output {
        self.m_add(rhs, Sign::Minus)?.as_vector()
    }
}

// Vector * [Matrix | Vector] = Matrix
impl<T: Num, M: Matrixified<Elem=T>> Mul<&M> for &Vector<T> {
    type Output = Result<Matrix<T>, MatrixifiedError>;

    fn mul(self, rhs: &M) -> Self::Output {
        self.m_mul(rhs)
    }
}

// Vector >>>
