#[macro_export]
macro_rules! remap_color{
    ($($old:expr => $r:expr,$g:expr,$b:expr),+ $(,)?) => {
        $(
            firefly_rust::set_color($old, firefly_rust::RGB{r: $r, g: $g, b: $b});
        )+
    }
}
