use std::collections::HashMap;
use std::io;
use std::convert::TryInto;

fn main() {
	let mut n=String::new();
	let mut reg=String::new();
	let mut name=String::new();
	let mut gender=String::new();
	let mut class=String::new();
	let mut pname=String::new();
	let mut address=String::new();
	let mut no=String::new();
	
	let mut v_reg:Vec<String>=Vec::new();
	let mut v_name:Vec<String>=Vec::new();
	let mut v_gender:Vec<String>=Vec::new();
	let mut v_class:Vec<String>=Vec::new();
	let mut v_pname:Vec<String>=Vec::new();
	let mut v_address:Vec<String>=Vec::new();
	let mut v_no:Vec<String>=Vec::new();
	
	let mut c=1;
	let mut choice=String::new();
	let mut ch=String::new();
	
	println!("____________________________________________WELCOME________________________________________________");
	
	println!("\nHow many record you want to store");
	io::stdin().read_line(&mut n).expect("Fail");
	let n:u32=n.trim().parse().expect("Fail");
	
	while c<=n{
		reg.clear();
		name.clear();
		gender.clear();
		class.clear();
		pname.clear();
		address.clear();
		no.clear();
		println!("\n.............................Enter Student Records.................................");
		println!("\nEnter Registration No:");
		io::stdin().read_line(&mut reg).expect("Fail");
		let reg:String=reg.trim().parse().expect("Failed");
		 if v_reg.contains(& reg)
        {
        
            println!(".....Registration No is already present, Please enter unique no.....");
            break;
        }
        else
		{
			println!("Enter name:");
			io::stdin().read_line(&mut name).expect("Fail");
			let name:String=name.trim().parse().expect("Failed");
		
			println!("Enter gender:");
			io::stdin().read_line(&mut gender).expect("Fail");
			let gender:String=gender.trim().parse().expect("Failed");
		
			println!("Enter class:");
			io::stdin().read_line(&mut class).expect("Fail");
			let class:String=class.trim().parse().expect("Failed");
		
			println!("Enter parent's name:");
			io::stdin().read_line(&mut pname).expect("Fail");
			let pname:String=pname.trim().parse().expect("Failed");
		
			println!("Enter address:");
			io::stdin().read_line(&mut address).expect("Fail");
			let address:String=address.trim().parse().expect("Failed");
		
			println!("Enter Mobile Number:");
			io::stdin().read_line(&mut no).expect("Fail");
			let no:String=no.trim().parse().expect("Failed");
			if no.len()==10
			{
            
			}
			else
			{
				println!("---Contact no Invalid, Please enter valid contact no---");
				break;      
			}
			v_reg.push(reg);
			v_name.push(name);
			v_gender.push(gender);
			v_class.push(class);
			v_pname.push(pname);
			v_address.push(address);
			v_no.push(no);
			c+=1;
	
			
		}
		    
	
	println!("\n____________________________Student Details___________________________________________");
	println!("\nReg No:{:?}\nName:{:?}\nGender:{:?}\nClass:{:?}\nParent's Name:{:?}\nAddress:{:?}\nMobile no:{:?}",v_reg,v_name,v_gender,v_class,v_pname,v_address,v_no);
	}
	
	
	let name:HashMap<&String,&String>=v_reg.iter()
	.zip(v_name.iter()).collect();
	
	let gender:HashMap<&String,&String>=v_reg.iter()
	.zip(v_gender.iter()).collect();
	
	let class:HashMap<&String,&String>=v_reg.iter()
	.zip(v_class.iter()).collect();
	
	let pname:HashMap<&String,&String>=v_reg.iter()
	.zip(v_pname.iter()).collect();
	
	let address:HashMap<&String,&String>=v_reg.iter()
	.zip(v_address.iter()).collect();
	
	let contact:HashMap<&String,&String>=v_reg.iter()
	.zip(v_no.iter()).collect();
	
	println!("\n{:?} {:?} {:?} {:?} {:?} {:?}",name,gender,class,pname,address,contact);
	
	
	println!("\n................Which Registration No to Search??.................\n");
	io::stdin().read_line(&mut choice).expect("Fail");
	let choice:String=choice.trim().parse().expect("Fail");
	
	
	name_s(name,&choice);
	gender_s(gender,&choice);
	class_s(class,&choice);
	pname_s(pname,&choice);
	address_s(address,&choice);
	contact_s(contact,&choice);
	
	println!("\n................Which Registration No to Delete??.................\n");
	io::stdin().read_line(&mut ch).expect("Fail");
	let ch:u32=ch.trim().parse().expect("Fail");
	
	 let s=ch.to_string();
    if v_reg.contains(&s)
    {
         let s= v_reg.remove(ch.try_into().unwrap());
         let s= v_name.remove(ch.try_into().unwrap());
         let s= v_gender.remove(ch.try_into().unwrap());
         let s= v_class.remove(ch.try_into().unwrap());
         let s= v_pname.remove(ch.try_into().unwrap());
		 let s= v_address.remove(ch.try_into().unwrap());
		 let s= v_no.remove(ch.try_into().unwrap());
    }

    println!("\nRecord is deleted successfully!!!!!");
	println!("\n....................Remaining records are....................");
    println!("\nRegistration ID: {:?}\nName: {:?}\nGender: {:?}\nClass: {:?}\nParent's Name: {:?}\nAddress: {:?}\nMobile No: {:?}",v_reg,v_name,v_gender,v_class,v_pname,v_address,v_no);
		
}

fn name_s(name:HashMap<&String,&String>,choice:&String){
		for (k,v) in name 
		{
			if k==choice
			{
				println!("\nName: {}",v);
			}
		}
}
fn gender_s(gender:HashMap<&String,&String>,choice:&String){
		for (k,v) in gender 
		{
			if k==choice
			{
				println!("Gender: {}",v);
			}
		}
}
fn class_s(class:HashMap<&String,&String>,choice:&String){
		for (k,v) in class
		{
			if k==choice
			{
				println!("Class: {}",v);
			}
		}
}
fn pname_s(pname:HashMap<&String,&String>,choice:&String){
		for (k,v) in pname
		{
			if k==choice
			{
				println!("Parent's name: {}",v);
			}
		}
}
fn address_s(address:HashMap<&String,&String>,choice:&String){
		for (k,v) in address 
		{
			if k==choice
			{
				println!("Address: {}",v);
			}
		}
}
fn contact_s(contact:HashMap<&String,&String>,choice:&String){
		for (k,v) in contact 
		{
			if k==choice
			{
				println!("Contact: {}",v);
			}
		}
}




