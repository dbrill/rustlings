// tests4.rs
//
// Make sure that we're testing for the correct conditions!
//
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a
// hint.
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

#[derive(Debug, PartialEq)]
struct RectangleModError{}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }


    // increases the height of the rectangle, returns RectangleModError if heigh goes negative or higher than 100
    pub fn add_height(&mut self, additional_height: i32) -> Result<& Self, RectangleModError>  {
	    if self.height + additional_height < 0 || self.height + additional_height > 100 {
		    Err(RectangleModError{})
	    } else {
		    self.height += additional_height;
		    Ok(self)
	    }
    } 
    // increases the height of the rectangle, returns RectangleModError if heigh goes negative or higher than 100
    pub fn add_width(&mut self, additional_width: i32) -> Result<& Self, RectangleModError> {
	    if self.width + additional_width < 0 || self.width + additional_width > 100 {
		    Err(RectangleModError{})
	    } else {
		    self.width += additional_width;
		    Ok(self)
	    }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // This test should check if the rectangle is the size that we pass into its constructor
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // check width
        assert_eq!(rect.height, 20); // check height
    }

    #[test]
    fn add_height() {
	let mut rect = Rectangle::new(10, 20);
	assert_eq!(rect.add_height(40).unwrap().height, 60);
    }
    #[test]
    fn add_too_much_height() {
	    let mut rect = Rectangle::new(10, 20);
	    assert_eq!(rect.add_height(100).err().unwrap(), RectangleModError{});
	    assert_eq!(rect.add_height(100).is_ok(), false); 
	    assert_eq!(rect.height, 20);
    }

    #[test]
    fn make_height_negative() {
	    let mut rect = Rectangle::new(10, 20);
	    assert_eq!(rect.add_height(-1000).err().unwrap(), RectangleModError{});
	    assert_eq!(rect.add_height(-1000).is_ok(), false);
	    assert_eq!(rect.height, 20);
    }
    #[test]
    fn add_width() {
	    let mut rect = Rectangle::new(10, 20);
	    assert_eq!(rect.add_width(50).unwrap().width, 60);
    }

    #[test]
    fn add_too_much_width() {
	    let mut rect = Rectangle::new(10, 20);
	    assert_eq!(rect.add_width(100).err().unwrap(), RectangleModError{});
	    assert_eq!(rect.add_width(100).is_ok(), false); 
	    assert_eq!(rect.width, 10);
    }

    #[test]
    fn make_width_negative() {
	    let mut rect = Rectangle::new(10, 20);
	    assert_eq!(rect.add_width(-1000).err().unwrap(), RectangleModError{});
	    assert_eq!(rect.add_width(-1000).is_ok(), false);
	    assert_eq!(rect.width, 10);
    }

    #[test]
    #[should_panic]
    fn negative_width() {
        // This test should check if program panics when we try to create rectangle with negative width
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic]
    fn negative_height() {
        // This test should check if program panics when we try to create rectangle with negative height
        let _rect = Rectangle::new(10, -10);
    }
}
