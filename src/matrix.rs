use core::fmt;
use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Deref, Mul},
    sync::mpsc::{self, Sender},
    thread,
};

use anyhow::{anyhow, Result};

pub struct Matrix<T: Debug> {
    data: Vec<T>,
    pub row: usize,
    pub col: usize,
}

const THREAD_NUM: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct MsgIn<T: Debug> {
    id: usize,
    row_v: Vector<T>,
    col_v: Vector<T>,
    sender: oneshot::Sender<MsgOut<T>>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct MsgOut<T: Debug> {
    id: usize,
    result: T,
}

#[allow(clippy::needless_borrows_for_generic_args)]
impl<T: Debug> MsgIn<T> {
    fn new(id: usize, row: Vector<T>, col: Vector<T>, sender: oneshot::Sender<MsgOut<T>>) -> Self {
        Self {
            id,
            row_v: row,
            col_v: col,
            sender,
        }
    }
}

#[allow(clippy::needless_borrows_for_generic_args)]
impl<T: Debug> MsgOut<T> {
    fn new(id: usize, result: T) -> Self {
        Self { id, result }
    }
}

fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T>
where
    T: Debug + Default + Copy + Add<Output = T> + AddAssign + Mul<Output = T>,
{
    if a.len() != b.len() {
        return Err(anyhow!("can't compute due different length"));
    }
    let mut result = T::default();
    for i in 0..a.len() {
        result += a[i] * b[i];
    }
    Ok(result)
}

impl<T> Mul for Matrix<T>
where
    T: Debug + Default + Copy + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    type Output = Matrix<T>;
    fn mul(self, rhs: Self) -> Self::Output {
        if let Ok(r) = multiply(&self, &rhs) {
            r
        } else {
            panic!("Multiply for Matrix error");
        }
    }
}

pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Debug + Default + Copy + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error: a.col != b.row"));
    }
    let mut receivers: Vec<oneshot::Receiver<MsgOut<T>>> = Vec::new();
    let senders: Vec<Sender<MsgIn<T>>> = (0..THREAD_NUM)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<MsgIn<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let v = dot_product(msg.row_v, msg.col_v);
                    match v {
                        Ok(result) => {
                            msg.sender
                                .send(MsgOut::new(msg.id, result))
                                .expect("send result error");
                        }
                        Err(e) => {
                            eprintln!("error: {}", e);
                        }
                    }
                }
            });
            tx
        })
        .collect::<Vec<_>>();
    println!("size: {}", a.col * b.row);
    let mut data = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let temp_b = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col = Vector::new(temp_b);
            let index = i * b.col + j;
            let (tx, rx) = oneshot::channel::<MsgOut<T>>();
            senders[index % THREAD_NUM]
                .send(MsgIn::new(index, row, col, tx))
                .expect("send failed");
            receivers.push(rx);
        }
    }
    for receiver in receivers {
        let r = receiver.recv()?;
        data[r.id] = r.result;
    }
    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

struct Vector<T: fmt::Debug> {
    vect: Vec<T>,
}

impl<T: Debug> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { vect: data.into() }
    }
}

impl<T: Debug> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.vect
    }
}

impl<T> fmt::Debug for Vector<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector({})", self)?;
        Ok(())
    }
}

impl<T> fmt::Display for Vector<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.vect.len() {
            write!(f, "{:?}", self.vect[i])?;
            if i < (self.vect.len() - 1) {
                write!(f, ",")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<T: Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for i in 0..self.row {
            write!(f, "[")?;
            for j in 0..self.col {
                write!(f, "{:?}", self.data[i * self.col + j])?;
                if j < (self.col - 1) {
                    write!(f, ",")?;
                }
            }
            write!(f, "]")?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Matrix(row = {}, col = {}, {})",
            self.row, self.col, self
        )
    }
}
