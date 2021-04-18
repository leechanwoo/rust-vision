
use opencv::core::
{ Mat, 
    MatTrait,
    Vector, 
    no_array, 
    Size,
    Scalar };


use opencv::videoio::
{ VideoCapture, 
    VideoWriter, 
    VideoWriterTrait, 
    VideoCaptureTrait,  
    CAP_ANY,
    CAP_FFMPEG,
    CAP_PROP_FOURCC
};


use opencv::features2d::
{ DrawMatchesFlags, 
    Feature2DTrait, 
    draw_keypoints };


use opencv::xfeatures2d::SIFT;
use datetime::LocalDateTime;

fn main() {
    let image = Mat::default();
    let out_img = Mat::default();
    let capture = VideoCapture::from_file("dance.mp4", CAP_FFMPEG);
    let sift = SIFT::create(0, 3, 0.04, 10.0, 1.6);
    let mut key_pts = Vector::new();
    let fourcc = VideoWriter::fourcc('m' as i8, 'p' as i8, '4' as i8, 'v' as i8);
    let recorder = match fourcc {
        Ok(_fourcc) => VideoWriter::new_with_backend("result.mp4", CAP_FFMPEG, _fourcc, 30.0, Size::new(640, 360), true),
        _ => panic!("Fail to read fourcc")
    };


    match (image, out_img, capture, sift, recorder) {
        (Ok(mut _image), 
         Ok(mut _out_img), 
         Ok(mut _capture), 
         Ok(mut _sift),
         Ok(mut _recorder)) => {

            match _recorder.is_opened() {
                Ok(true) => loop {
                    match _capture.read(&mut _image) {
                        Ok(true) => {
                            _sift.detect(&mut _image, &mut key_pts, &mut no_array().unwrap());
                            draw_keypoints(&mut _image, &key_pts, &mut _out_img, Scalar::new(255.0, 0.0, 0.0, 0.0), DrawMatchesFlags::DEFAULT);
                            match _recorder.write(&_out_img) {
                                Ok(_) => println!("{:?}, {:?}, {:?}", LocalDateTime::now(), _out_img.mat_size(), key_pts.len()),
                                _ => {
                                    panic!("Fail to write image");
                                }
                            }
                        }
                        _ => {  
                            _recorder.release();
                            break;
                        }
                    }
                },
                _ => println!("Writer is not opened")
            }
        },

        _ => panic!("something goes wrong")
    }
}

