pub const VEC3_LEN: usize = 3;

#[derive(PartialEq, Debug)]
pub struct Vec3([i32; VEC3_LEN]);

impl Vec3 {
    pub fn default_vec3() -> Vec3 {
        Self([0; VEC3_LEN])
    }

    pub fn vec3_scalar_sum(&self, other: &Vec3) -> i32 {
        let mut c: i32 = 0;
        for (i, e) in self.0.iter().enumerate() {
            let r = other.0[i];
            //PLS Help: почему сложение работает для `&i32 + i32`
            c += e + r;
        }
        c
    }

    //PLS Help
    //возножно ли вернуть Self (не ссылку) не копируя ?
    pub fn vec3_vector_sum(&mut self, other: &Vec3) -> &Self {
        for (i, e) in self.0.iter_mut().enumerate() {
            let r = other.0[i];
            //PLS Help: а здесь сложение не работает без разименования
            *e += r;
        }
        self
    }
}

impl From<[i32; VEC3_LEN]> for Vec3 {
    fn from(value: [i32; VEC3_LEN]) -> Self {
        Vec3(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_vec3_should_return_zero_vec() {
        let result: Vec3 = Vec3::default_vec3();
        assert_eq!(result, [0, 0, 0].into());
    }

    #[test]
    fn vec3_vector_sum_should_return_unit_vector() {
        let mut temp_vec3 = Vec3::default_vec3();
        //PLS Help: как можно использовать цепочку вызовов:
        //let result = Vec3::default_vec3().vec3_vector_sum(&mut [1, 1, 1].into());
        let result = temp_vec3.vec3_vector_sum(&mut [1, 1, 1].into());
        assert_eq!(result, &[1, 1, 1].into());
    }

    #[test]
    fn vec3_scalar_sum_should_return_3_for_unit_vector() {
        let result: i32 = Vec3::default_vec3().vec3_scalar_sum(&[1, 1, 1].into());
        assert_eq!(result, 3);
    }
}
