extern crate iron;
#[macro_use] extern crate mime;

extern crate urlencoded;
extern crate router;
use iron::prelude::*;
use iron::status;
use router::Router;
use urlencoded::UrlEncodedBody;
use std::str::FromStr;

fn get_form(_request : &mut Request) -> IronResult<Response>{
    let mut response= Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(r#"
    <title> "Addition Calculator" </title>
    <form action="/add" method="post">
        <input type="text" name="n"/>
        <input type="text" name="n"/>
        <button type= "submit">Compute addition</button>
    </form>"#);

    return Ok(response);
}

fn add(i: u64, j: u64) -> u64{
    i + j
}


fn post_add(request : & mut Request) -> IronResult<Response>{
    let mut response = Response::new();
    let form_data = match request.get_ref::<UrlEncodedBody>(){
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing from data: {:?}\n",e));
            return Ok(response);
        }

        Ok(map) => map
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }

        Some(nums) => nums
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed){
            Err(_) => {
                response.set_mut(status::BadRequest);
                return Ok(response);
            }
            Ok(n)=> { numbers.push(n);}
        }
    }

    let mut d = numbers[0];

    for m in &numbers[1..] {
        d = add(d,*m);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(format!("The sum of the number {:?} is <b>{}</b>\n",numbers,d));
    Ok(response)
}

fn main() {

    let mut router = Router::new();
    router.get("/",get_form, "root");
    router.post("/add", post_add, "add");


    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}
