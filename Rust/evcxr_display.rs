use std::fmt::Debug;

pub struct Matrix<T> {
    pub values: Vec<T>,
    pub row_size: usize,
}

impl<T: Debug> Matrix<T> {
    pub fn evcxr_display(&self) {
        let mut html = String::new();
        html.push_str("<table>");

        for r in 0..(self.values.len() / self.row_size) {
            html.push_str("<tr>");

            for c in 0..self.row_size {
                html.push_str("<td>");
                html.push_str(&format!("{:?}", self.values[r * self.row_size + c]));
                html.push_str("</td>");
            }

            html.push_str("</tr>");
        }

        html.push_str("</table>");
        println!("EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT", html);
    }
}
