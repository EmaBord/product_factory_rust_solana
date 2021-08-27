use anchor_lang::prelude::*;

#[program]
pub mod products {
    use super::*;
    
    #[state]
    pub struct ProductFactory{
        pub products: Vec<Product>,
    }

    
    impl ProductFactory {
        pub fn new(_ctx: Context<Empty>) -> Result<Self> {
            let mut whitelist = vec![];
            whitelist.resize(10, Product::default());
            Ok(Self {
                products:whitelist,

            })

        }

        pub fn init(&mut self, _ctx:Context<Empty>)-> Result<()>{
            self.products.clear();

            Ok(())
        }
    
        pub fn create_product(
            &mut self,
            _ctx: Context<Empty>,
            _name:String,
            _owner: Pubkey
        ) -> Result<()>{
            
            
            if self.products.len() == 10 {
                return Err(ErrorCode::MaximumSize.into());
            }

            let p = Product::new(
                _name,
                _owner,
            );

            self.products.push(p);
            Ok(())
        }

        pub fn delegate_product(
            &mut self,
            _ctx: Context<Empty>,
            _pid:u8,
            _owner: Pubkey,
            _delegate_to: Pubkey
        ) -> Result<()>{

            let p = & mut self.products[usize::from(_pid)];
            
            if p.owner != _owner {
                return Err(ErrorCode::InvalidOwner.into());
            }

            if p.status != 0 {
                return Err(ErrorCode::InvalidStatus.into());
            }

            if _owner == _delegate_to {
                return Err(ErrorCode::InvalidDelegate.into());
            }

            p.delegate_to(_delegate_to);

            Ok(())
        }

        pub fn accept_product(
            &mut self,
            _ctx: Context<Empty>,
            _pid:u8,
            _delegate_to: Pubkey
        ) -> Result<()>{

            let  p = & mut self.products[usize::from(_pid)];

            let d_t: Option<Pubkey> = Some(_delegate_to);

            if p.status != 1 {
                return Err(ErrorCode::InvalidStatus.into());
            }

            if Some(p.delegate_to) != Some(d_t) {
                return Err(ErrorCode::InvalidDelegate.into());
            }

            p.accept(_delegate_to);
            

            Ok(())
        }

    }
}

#[derive(Accounts)]
pub struct Empty {}

#[error]
pub enum ErrorCode {
    #[msg("MaximumSize!")]
    MaximumSize,
    #[msg("InvalidOwner!")]
    InvalidOwner,
    #[msg("InvalidStatus!")]
    InvalidStatus,
    #[msg("InvalidDelegate!")]
    InvalidDelegate,
}

#[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Default, Clone)]
pub struct Product{
        pub status: u8,
        pub name: String,
        pub owner:Pubkey, 
        pub delegate_to:Option<Pubkey>,
}
impl Product {
    pub fn new(
        name: String,
        owner: Pubkey,
    ) -> Product{
        Product { 
            status: 0,
            name: name,
            owner:owner,
            delegate_to:None
        } 
    }

    pub fn default() -> Product{
        Product { 
            status: 5,
            name: "None".to_string(),
            owner:Pubkey::default(),
            delegate_to:None
        }
    } 
}

impl Product {
    pub fn get_owner(&mut self) -> Pubkey{
        self.owner    
    } 
}


impl Product {
    pub fn get_name(&mut self) -> String{
        self.name.to_string()
    } 
}

impl Product {
    pub fn get(& self) -> Product{
        Product{
            name:self.name.to_string(),
            status:self.status,
            owner:self.owner,
            delegate_to:self.delegate_to
        }    
    } 
}

impl Product {
    pub fn get_delegate(&mut self) -> Option<Pubkey>{
        self.delegate_to    
    } 
}

impl Product {
    pub fn get_status(&mut self) -> u8{
        self.status
    } 
}

impl Product {
    pub fn delegate_to(&mut self, delegate: Pubkey){
        self.status = 1;
        self.delegate_to = Some(delegate);
    } 
}

impl Product {
    pub fn accept(&mut self, delegate: Pubkey){
        self.status = 0;
        self.owner = delegate;
        self.delegate_to = None;
    } 
}




