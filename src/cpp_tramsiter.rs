

#[cxx::bridge]
mod share_file_stucts{

    struct package{

    }
    struct BlobMetaData{
        size : usize,
        tags : Vec<string>,
        last_trazmtion : u64,
         }

}

extern "Rust"{
    type MultiBuf;

    fn next_chuck(buf: mut MultiBuf) -> &[u8];

}

unsafe extern "C++"
{
    include!("pathto.header.h");

    type listofC__object_movable_feild_hiden_in_rust;
// c++ functions implied on other side.
    fn tag
    fn metadata()
}
