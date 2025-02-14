use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use barcoders::generators::image::Image;
use barcoders::sym::code39::Code39;
use barcoders::sym::ean13::{Bookland, EAN13};
use barcoders::sym::ean8::EAN8;
use barcoders::sym::ean_supp::EANSUPP;
use image::RgbaImage;
use slint::SharedPixelBuffer;
use regex::Regex;
slint::include_modules!();


struct BarcodeCheck{}

impl BarcodeCheck{
    fn barcode_checker(barcode_type:String,barcode_value:String)-> Vec<u8>
    {
        if barcode_type == "Code39".to_string()
        {
          return  Code39::new(format!("{barcode_value}")).unwrap().encode()
        }
        else if barcode_type == "EAN13".to_string()
        {
            return EAN13::new(format!("{barcode_value}")).unwrap().encode()
        }
        else if barcode_type == "Bookland".to_string()
        {
            return Bookland::new(format!("{barcode_value}")).unwrap().encode()
        }
        else if barcode_type == "EAN8".to_string()
        {
            return EAN8::new(format!("{barcode_value}")).unwrap().encode()
        }
        else if barcode_type == "EANSUPP".to_string()
        {
            return EANSUPP::new(format!("{barcode_value}")).unwrap().encode()
        }
        
        Code39::new(format!("{barcode_value}")).unwrap().encode()
    }

    fn barcode_limit_size(barcode_type:String)->usize
    {
        if barcode_type == "Code39".to_string()
        {
            return 49
        }
        //Used for EAN 13 and JAN
        else {
            return 12
        }
    }

    fn barcode_regex_type(barcode_type:String) ->Regex
    {
        if barcode_type == "Code39"
        {
            return Regex::new(r"[^A-Z0-9-$%./+\s]").unwrap()// This matches anything that's not uppercase or a number
        }
        else 
        {
            return Regex::new(r"[^0-9]").unwrap() // This matches anything thats a number
        }
    }
}


pub fn main()
{
    
  	let state = init();
  	let main_window = state.main_window.clone_strong();
    main_window.run().unwrap();

}


pub struct State {
    pub main_window: MainWindow,
    // pub todo_model: Rc<slint::VecModel<TodoItem>>,
}


fn init() -> State
{
    //init the window    
    let main_window = MainWindow::new().unwrap();

    //////////////////////////////////////////

    //here we will set up the button function

    /////////////////////////////////////////
    
    
    /////////////////////////////////////////////////////
    
    //generate the barcode preview
    
    /////////////////////////////////////////////////////
    main_window.on_barcode_preview_btn({
        let main_window = main_window.clone_strong();
        move|user_name,barcodes_input|{
           // encoded_image = generate_barcode(name.trim().to_string(), id.trim().to_string(), pass.trim().to_string());
            
           main_window.set_barcode_length( barcodes_input.trim().to_string().len() as i32);
           
            //if the barcode_input is not empty update barcode_value
            //also update the display text if it exist
            if barcodes_input.trim().to_string() != "".to_string()
            {
                
                main_window.set_the_barcode_value(barcodes_input.into());
                main_window.set_user_name(user_name.into());
             
            }
        
        }
    
    }
);

/////////////////////////////////////////////////////

//Export the barcode to file

/////////////////////////////////////////////////////
main_window.on_barcode_generate_btn({
    let main_window = main_window.clone_strong();
    move|user_name,barcodes_input|{
       // encoded_image = generate_barcode(name.trim().to_string(), id.trim().to_string(), pass.trim().to_string());
       
        //if the barcode_input is not empty update barcode_value
        //also update the display text if it exist
        if !barcodes_input.trim().to_string().is_empty()
        {
            if main_window.get_barcode_combo_value().trim().to_string() == "Code39".to_string()
            {
                generate_barcode(main_window.get_barcode_combo_value().clone().to_string(),user_name.trim().to_string(),barcodes_input.trim().to_string());
            }
            else {
                if  barcodes_input.trim().to_string().len() > BarcodeCheck::barcode_limit_size(main_window.get_barcode_combo_value().trim().to_string())
                {
                    generate_barcode(main_window.get_barcode_combo_value().clone().to_string(),user_name.trim().to_string(),barcodes_input.trim().to_string());
                }
            }
        }
    }

}
);


    /////////////////////////////////////////////////////
    
    //Limits the input custom
    
    /////////////////////////////////////////////////////
    main_window.on_input_detected({
        let main_window = main_window.clone_strong();
        move || {
            let current_text = main_window.get_filtered_text();
            
            // Regex to allow only uppercase letters, numbers, whitespace, and ASCII characters
            // Replace any characters not matching the allowed pattern
            let filtered = BarcodeCheck::barcode_regex_type(main_window.get_barcode_combo_value().trim().to_string()).replace_all(&current_text,"").to_string();
            //let filtered = regex_type.replace_all(&current_text, "").to_string();
            if current_text.trim().to_string() != "".to_string()
            {
                if filtered != current_text.to_string() {
    
                
                    main_window.set_filtered_text(filtered.into());
                            
                }
               
               //limits the text to the max size only
                if main_window.get_filtered_text().trim().len() as usize > BarcodeCheck::barcode_limit_size(main_window.get_barcode_combo_value().trim().to_string())  
                {
                    
                    main_window.set_filtered_text(main_window.get_filtered_text().trim()[..(main_window.get_filtered_text().trim().len() as usize) -1].into());
                }
            }

        }
    });



    ///////////////////////////////////////////////////////
    
    //Image preview is generated here
    
    //////////////////////////////////////////////////////
    main_window.on_generate_image(
        {
            let main_window = main_window.clone_strong();
            move|barcode_text|{
        

            let encoded_image = BarcodeCheck::barcode_checker(main_window.get_barcode_combo_value().clone().to_string(),barcode_text.trim().to_string());
        
            let buffered = Image::image_buffer(100);
            
            let img = buffered.generate_buffer(&encoded_image[..]).unwrap();

            // Convert DynamicImage to RGBA8 format (compatible with Slint)
            let slint_image = convert_rgba_to_slint_image(img);
            
            slint_image
     
    }});

    ///////////////////////////////////////////

    //here we will add all of the possible features based on State

    ///////////////////////////////////////////
    State{main_window}

}




/***********************************************************************
 * 
 * 
 * Name: generate_barcode
 * Description: export the barcode to file. It will always create in
 * the same directory as the executable.
 * 
 * 
 * 
 ************************************************************************/
fn generate_barcode(barcode_type:String,name:String,barcode_value:String)
{
    let encoded =  BarcodeCheck::barcode_checker(barcode_type,barcode_value);
    let png = Image::png(80);
    let bytes = png.generate(&encoded[..]).unwrap();
    let file = File::create(&Path::new(&format!("./{name}_barcode.png"))).unwrap();
    let mut writer = BufWriter::new(file);
    writer.write(&bytes[..]).unwrap();
}


/***********************************************************************
 * 
 * 
 * Name: convert_rgba_to_slint_image
 * Description: Generate the image for preview
 * 
 * 
 * 
 ************************************************************************/
fn convert_rgba_to_slint_image(rgba_image: RgbaImage) -> slint::Image {
    // Extract pixel data and dimensions
    let rgba_data = rgba_image.as_flat_samples();
    let pixel_data = rgba_data.samples; // Flat RGBA8 byte array
    let width = rgba_image.width();
    let height = rgba_image.height();

    // Create a SharedPixelBuffer
    let pixel_buffer = SharedPixelBuffer::clone_from_slice(
        pixel_data,
        width,
        height,
    );

    // Convert to Slint Image
    slint::Image::from_rgba8(pixel_buffer)
}

