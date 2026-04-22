

#[derive(Debug)]
struct Float {
    value: f32,
}

#[derive(Debug)]
struct Complexe {
    re: f32,
    im: f32,
}


// oxxxxx[====== VECTOR ========================>

pub struct Vector<K> {
    size: usize,
    data: *mut K,
}

impl<K> Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size, "Index out of bounds");
        unsafe { &*self.data.add(index) }
    }
}

impl<K>  IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.size, "Index out of bounds");
        unsafe { &mut *self.data.add(index) }
    }
}


// oxxxxx[====== MATRIX ========================>

pub struct Matrix<K> {
    rows: usize,
    cols: usize,
    data: *mut K,
}

impl<K> Index<(usize, usize)> for Matrix<K> {
    type Output = K;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        assert!(row < self.rows, "Row index out of bounds");
        assert!(col < self.cols, "Column index out of bounds");

        let offset = row * self.cols + col;
        unsafe { &*self.data.add(offset) }
    }
}

impl<K> IndexMut<(usize, usize)> for Matrix<K> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        assert!(row < self.rows, "Row index out of bounds");
        assert!(col < self.cols, "Column index out of bounds");

        let offset = row * self.cols + col;
        unsafe { &mut *self.data.add(offset) }
    }
}
