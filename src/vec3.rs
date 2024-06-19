pub const VEC3_LEN: usize = 3;

pub type Vec3 = [i32; VEC3_LEN];

pub fn default_vec3() -> Vec3 {
    [0; VEC3_LEN]
}

pub fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
    let mut c: Vec3 = default_vec3();
    for i in 0..VEC3_LEN {
        c[i] = a[i] + b[i];
    }
    c
}

pub fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
    let mut c: i32 = 0;
    for i in 0..VEC3_LEN {
        c += a[i] + b[i];
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_vec3_should_return_zero_vec() {
        let result: Vec3 = default_vec3();
        assert_eq!(result, [0, 0, 0]);
    }

    #[test]
    fn vec3_vector_sum_should_return_unit_vector() {
        let result: Vec3 = vec3_vector_sum(default_vec3(), [1, 1, 1]);
        assert_eq!(result, [1, 1, 1]);
    }

    #[test]
    fn vec3_scalar_sum_should_return_3_for_unit_vector() {
        let result: i32 = vec3_scalar_sum(default_vec3(), [1, 1, 1]);
        assert_eq!(result, 3);
    }
}
