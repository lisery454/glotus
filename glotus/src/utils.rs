use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    hash::Hash,
    rc::Rc,
};

use cgmath::{Matrix, Matrix3, Matrix4, SquareMatrix};

fn get_mapped_refmut<T, U, F>(cell: &RefCell<T>, mapper: F) -> RefMut<U>
where
    F: FnOnce(&mut T) -> &mut U,
{
    RefMut::map(cell.borrow_mut(), mapper)
}

pub fn compute_normal_matrix(model_matrix: &Matrix4<f32>) -> Matrix3<f32> {
    // 1. 计算模型矩阵的逆矩阵
    let inverse_model_matrix = model_matrix
        .invert()
        .expect("Model matrix must be invertible for normal transformation");

    // 2. 取逆矩阵的左上 3x3 部分
    let inverse_model_3x3 = Matrix3::new(
        inverse_model_matrix[0][0],
        inverse_model_matrix[0][1],
        inverse_model_matrix[0][2],
        inverse_model_matrix[1][0],
        inverse_model_matrix[1][1],
        inverse_model_matrix[1][2],
        inverse_model_matrix[2][0],
        inverse_model_matrix[2][1],
        inverse_model_matrix[2][2],
    );

    // 3. 转置 3x3 矩阵得到法线矩阵
    inverse_model_3x3.transpose()
}
