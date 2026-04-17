pub trait VecSplit {
    type Output;
    fn split(self) -> Self::Output;
}

impl<T0, T1> VecSplit for Vec<(T0, T1)> {
    type Output = (Vec<T0>, Vec<T1>);
    fn split(self) -> Self::Output {
        let mut res = (vec![], vec![]);
        for e in self { res.0.push(e.0); res.1.push(e.1); }
        res
    }
}

impl<T0, T1, T2> VecSplit for Vec<(T0, T1, T2)> {
    type Output = (Vec<T0>, Vec<T1>, Vec<T2>);
    fn split(self) -> Self::Output {
        let mut res = (vec![], vec![], vec![]);
        for e in self { res.0.push(e.0); res.1.push(e.1); res.2.push(e.2); }
        res
    }
}

// impl<T0, T1, T2, T3> VecSplit for Vec<(T0, T1, T2, T3)> {
//     type Output = (Vec<T0>, Vec<T1>, Vec<T2>, Vec<T3>);
//     fn split(self) -> Self::Output {
//         let mut res = (vec![], vec![], vec![], vec![]);
//         for e in self { res.0.push(e.0); res.1.push(e.1); res.2.push(e.2); res.3.push(e.3); }
//         res
//     }
// }
