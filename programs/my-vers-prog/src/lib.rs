use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod my_vers_prog {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn initialize2(ctx: Context<Initialize2>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:
    pub play0: AccountInfo<'info>,
    /// CHECK:
    pub play1: AccountInfo<'info>,
    /// CHECK:
    pub play2: AccountInfo<'info>,
    /// CHECK:
    pub play3: AccountInfo<'info>,
    /// CHECK:
    pub play4: AccountInfo<'info>,
    /// CHECK:
    pub play5: AccountInfo<'info>,
    /// CHECK:
    pub play6: AccountInfo<'info>,
    /// CHECK:
    pub play7: AccountInfo<'info>,
    /// CHECK:
    pub play8: AccountInfo<'info>,
    /// CHECK:
    pub play9: AccountInfo<'info>,
    /// CHECK:
    pub play10: AccountInfo<'info>,
    /// CHECK:
    pub play11: AccountInfo<'info>,
    /// CHECK:
    pub play12: AccountInfo<'info>,
    /// CHECK:
    pub play13: AccountInfo<'info>,
    /// CHECK:
    pub play14: AccountInfo<'info>,
    /// CHECK:
    pub play15: AccountInfo<'info>,
    /// CHECK:
    pub play16: AccountInfo<'info>,
    /// CHECK:
    pub play17: AccountInfo<'info>,
    /// CHECK:
    pub play18: AccountInfo<'info>,
    /// CHECK:
    pub play19: AccountInfo<'info>,
    /// CHECK:
    pub play20: AccountInfo<'info>,
    /// CHECK:
    pub play21: AccountInfo<'info>,
    /// CHECK:
    pub play22: AccountInfo<'info>,
    /// CHECK:
    pub play23: AccountInfo<'info>,
    /// CHECK:
    pub play24: AccountInfo<'info>,
//     // --------------------------------------- if you go above ~25 more or less it starts to error. why?
//     /// CHECK:
//     pub play25: AccountInfo<'info>,
//     /// CHECK:
//     pub play26: AccountInfo<'info>,
//     /// CHECK:
//     pub play27: AccountInfo<'info>,
//     /// CHECK:
//     pub play28: AccountInfo<'info>,
//     /// CHECK:
//     pub play29: AccountInfo<'info>,
//     /// CHECK:
//     pub play30: AccountInfo<'info>,
//     /// CHECK:
//     pub play31: AccountInfo<'info>,
//     /// CHECK:
//     pub play32: AccountInfo<'info>,
//     /// CHECK:
//     pub play33: AccountInfo<'info>,
//     /// CHECK:
//     pub play34: AccountInfo<'info>,
//     /// CHECK:
//     pub play35: AccountInfo<'info>,
//     /// CHECK:
//     pub play36: AccountInfo<'info>,
//     /// CHECK:
//     pub play37: AccountInfo<'info>,
//     /// CHECK:
//     pub play38: AccountInfo<'info>,
//     /// CHECK:
//     pub play39: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Initialize2<'info> {
    /// CHECK:
    pub play0: AccountInfo<'info>,
    /// CHECK:
    pub play1: AccountInfo<'info>,
    /// CHECK:
    pub play2: AccountInfo<'info>,
    /// CHECK:
    pub play3: AccountInfo<'info>,
    /// CHECK:
    pub play4: AccountInfo<'info>,
    /// CHECK:
    pub play5: AccountInfo<'info>,
    /// CHECK:
    pub play6: AccountInfo<'info>,
    /// CHECK:
    pub play7: AccountInfo<'info>,
    /// CHECK:
    pub play8: AccountInfo<'info>,
    /// CHECK:
    pub play9: AccountInfo<'info>,
    /// CHECK:
    pub play10: AccountInfo<'info>,
    /// CHECK:
    pub play11: AccountInfo<'info>,
    /// CHECK:
    pub play12: AccountInfo<'info>,
    /// CHECK:
    pub play13: AccountInfo<'info>,
    /// CHECK:
    pub play14: AccountInfo<'info>,
    /// CHECK:
    pub play15: AccountInfo<'info>,
    /// CHECK:
    pub play16: AccountInfo<'info>,
    /// CHECK:
    pub play17: AccountInfo<'info>,
    /// CHECK:
    pub play18: AccountInfo<'info>,
    /// CHECK:
    pub play19: AccountInfo<'info>,
    /// CHECK:
    pub play20: AccountInfo<'info>,
    /// CHECK:
    pub play21: AccountInfo<'info>,
    /// CHECK:
    pub play22: AccountInfo<'info>,
    /// CHECK:
    pub play23: AccountInfo<'info>,
    /// CHECK:
    pub play24: AccountInfo<'info>,
    // // --------------------------------------- if you go above ~25 more or less it starts to error. why?
    // /// CHECK:
    // pub play25: AccountInfo<'info>,
    // /// CHECK:
    // pub play26: AccountInfo<'info>,
    // /// CHECK:
    // pub play27: AccountInfo<'info>,
    // /// CHECK:
    // pub play28: AccountInfo<'info>,
    // /// CHECK:
    // pub play29: AccountInfo<'info>,
    // /// CHECK:
    // pub play30: AccountInfo<'info>,
    // /// CHECK:
    // pub play31: AccountInfo<'info>,
    // /// CHECK:
    // pub play32: AccountInfo<'info>,
    // /// CHECK:
    // pub play33: AccountInfo<'info>,
    // /// CHECK:
    // pub play34: AccountInfo<'info>,
    // /// CHECK:
    // pub play35: AccountInfo<'info>,
    // /// CHECK:
    // pub play36: AccountInfo<'info>,
    // /// CHECK:
    // pub play37: AccountInfo<'info>,
    // /// CHECK:
    // pub play38: AccountInfo<'info>,
    // /// CHECK:
    // pub play39: AccountInfo<'info>,
}
