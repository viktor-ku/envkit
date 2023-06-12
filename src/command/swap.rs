use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

pub fn swap(a_path: PathBuf, b_path: PathBuf) -> std::io::Result<()> {
    let a_contents = read_to_string(&a_path)?;
    let b_contents = read_to_string(&b_path)?;

    write(a_path, b_contents)?;
    write(b_path, a_contents)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn simple_swap() {
        let a1 = fs::read_to_string("examples/one.env").unwrap();
        let b1 = fs::read_to_string("examples/one.template.env").unwrap();

        swap(
            "examples/one.env".into(),
            "examples/one.template.env".into(),
        )
        .unwrap();

        let a2 = fs::read_to_string("examples/one.env").unwrap();
        let b2 = fs::read_to_string("examples/one.template.env").unwrap();

        assert_eq!(a1, b2);
        assert_eq!(b1, a2);

        swap(
            "examples/one.env".into(),
            "examples/one.template.env".into(),
        )
        .unwrap();

        let a3 = fs::read_to_string("examples/one.env").unwrap();
        let b3 = fs::read_to_string("examples/one.template.env").unwrap();

        assert_eq!(a3, a1);
        assert_eq!(b3, b1);
    }
}
