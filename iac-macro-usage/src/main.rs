use iac_macro::iac;

fn main() {
    iac!(
        bucket(name=uniquename)
    );
    iac!{
        lambda(
            name=a_name
        )
    }
    iac!(
        lambda (name=my_name,mem=1024,time=15)
    );
    iac![
        lambda (name=name) bucket(name=uniquename)
    ];
    iac!(
        bucket(name=uniquename) => lambda (name=anothername)
    );
    iac!{
        bucket(name=b) => lambda (name=l,mem=1024,time=15)
    }
}
