use std::error::Error;
use std::fmt::Display;

// error with unit struct
#[derive(Debug)]
struct NumberNotPositiveError;

impl Display for NumberNotPositiveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Number should be positive!")
    }
}
impl Error for NumberNotPositiveError {}


// error with enum
#[derive(Debug)]
enum NumberError {
    NotPositive,
    NotNegative,
    Others(String)
}

impl Error for NumberError {}
impl Display for NumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let errmsg: String;
        match self {
            Self::NotPositive => errmsg = "Number should be positive".into(),
            Self::NotNegative => errmsg = "Number should be negative".into(),
            Self::Others(e) => errmsg = e.clone(),
        }
        write!(f, "{errmsg}")
    }
}

// demo
// argument number pada fungsi ini harus bilangan bulat/integer
// antara 0 s/d 20, jika tidak fungsi ini akan menghasilakan error
fn getval(number: i32) -> Result<u32, Box<dyn Error>> {
    // if number > -1 {
    //     Ok(number as u32)
    // } 
    // else {
    //     Err(Box::new(NumberNotPositiveError))
    // }
    if number > 20 {
        let error_text = "Number is not smaller than 21";
        Err(Box::new(NumberError::Others(error_text.into())))
    }
    else if number < 0 {
        Err(Box::new(NumberError::NotPositive))
    }
    else {
        Ok(number as u32)
    }
}

pub fn err_handling() -> Result<u32, Box<dyn Error>> {
    // let n = getval(-23)?;
    // println!("{n}");
    // Ok(n)

    // match getval(-23) {
    //     Ok(n) => {
    //         println!("{n}");
    //         Ok(n)
    //     },
    //     Err(err) => {
    //         Err(err)
    //     }
    // }
    
    let mut result;

    // match getval(-32) {
    //     Ok(n) => {
    //         result = n;
    //         println!("{result}");
    //     },
    //     Err(err) => return Err(err)
    // }

    // match getval(32) {
    //     Ok(n) => {
    //         result = n;
    //         println!("{result}");
    //     },
    //     Err(err) => return Err(err)
    // }

    // match getval(12) {
    //     Ok(n) => {
    //         result = n;
    //         println!("{result}");
    //     },
    //     Err(err) => return Err(err)
    // }

    result = getval(-32)?; println!("{result}");
    result = getval(32)?; println!("{result}");
    result = getval(-12)?; println!("{result}");

    Ok(result)


    // let ok_result = getval(-32)?;
    // println!("{ok_result}");



    // match getval(23) {
    //     Ok(n) => println!("{n}"),
    //     Err(err) => println!("{err}")
    // }

    // match getval(13) {
    //     Ok(n) => println!("{n}"),
    //     Err(err) => println!("{err}")
    // }
}


fn option_demo() -> Option<u32> {
    let opt = None;

    
    let o = opt?;
    
    Some(o)
}

