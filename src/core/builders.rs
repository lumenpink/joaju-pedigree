use crate::core::{
    AdoptionType, Individual, Ped, PedError, PregnancyLossType, Sex, Status, TwinType,
};

impl Individual {
    pub fn founder(id: &str, sex: Sex) -> Self {
        Self {
            id: id.to_string(),
            fid: None,
            mid: None,
            sex,
            status: Status::Alive,
            proband: false,
            affected: 0,
            carrier: false,
            suspect: false,
            twin_group: None,
            twin_type: TwinType::None,
            label: None,
            pregnancy_loss: PregnancyLossType::None,
            adoption: AdoptionType::None,
        }
    }

    pub fn child(id: &str, fid: &str, mid: &str, sex: Sex) -> Self {
        Self {
            id: id.to_string(),
            fid: Some(fid.to_string()),
            mid: Some(mid.to_string()),
            sex,
            status: Status::Alive,
            proband: false,
            affected: 0,
            carrier: false,
            suspect: false,
            twin_group: None,
            twin_type: TwinType::None,
            label: None,
            pregnancy_loss: PregnancyLossType::None,
            adoption: AdoptionType::None,
        }
    }
}

impl Ped {
    /// Gera um pedigree nuclear com pai (1), mãe (2) e `nch` filhos.
    pub fn nuclear(nch: usize) -> Result<Self, PedError> {
        let mut ped = Ped::new();
        ped.add(Individual::founder("1", Sex::Male))?;
        ped.add(Individual::founder("2", Sex::Female))?;

        for i in 1..=nch {
            let id = (2 + i).to_string();
            let sex = if i % 2 == 1 { Sex::Male } else { Sex::Female };
            ped.add(Individual::child(&id, "1", "2", sex))?;
        }
        Ok(ped)
    }

    /// Gera meio-irmãos com 1 pai comum (2) e 2 mães (1, 3).
    pub fn half_sibs(nch1: usize, nch2: usize) -> Result<Self, PedError> {
        let mut ped = Ped::new();
        ped.add(Individual::founder("1", Sex::Female))?;
        ped.add(Individual::founder("2", Sex::Male))?;
        ped.add(Individual::founder("3", Sex::Female))?;

        let mut curr_id = 4;
        for _ in 0..nch1 {
            ped.add(Individual::child(&curr_id.to_string(), "2", "1", Sex::Male))?;
            curr_id += 1;
        }
        for _ in 0..nch2 {
            ped.add(Individual::child(&curr_id.to_string(), "2", "3", Sex::Male))?;
            curr_id += 1;
        }
        Ok(ped)
    }

    /// Gera primos de primeiro grau (grau 1).
    pub fn cousins(_degree: usize) -> Result<Self, PedError> {
        let mut ped = Ped::new();
        // Avós: 1 (M), 2 (F)
        ped.add(Individual::founder("1", Sex::Male))?;
        ped.add(Individual::founder("2", Sex::Female))?;
        // Filhos dos avós: 3 (M), 4 (M)
        ped.add(Individual::child("3", "1", "2", Sex::Male))?;
        ped.add(Individual::child("4", "1", "2", Sex::Male))?;
        // Cônjuges externos: 5 (F), 6 (F)
        ped.add(Individual::founder("5", Sex::Female))?;
        ped.add(Individual::founder("6", Sex::Female))?;
        // Primos: 7 (filho de 3 e 5), 8 (filho de 4 e 6)
        ped.add(Individual::child("7", "3", "5", Sex::Male))?;
        ped.add(Individual::child("8", "4", "6", Sex::Female))?;
        Ok(ped)
    }

    /// Gera pedigree avuncular (tio e sobrinha).
    pub fn avuncular() -> Result<Self, PedError> {
        let mut ped = Ped::new();
        ped.add(Individual::founder("1", Sex::Male))?;
        ped.add(Individual::founder("2", Sex::Female))?;
        ped.add(Individual::child("3", "1", "2", Sex::Male))?;
        ped.add(Individual::child("4", "1", "2", Sex::Male))?;
        ped.add(Individual::founder("5", Sex::Female))?;
        ped.add(Individual::child("6", "4", "5", Sex::Female))?;
        Ok(ped)
    }

    /// Gera linhagem linear de `generations` gerações sucessivas.
    pub fn linear(generations: usize) -> Result<Self, PedError> {
        let mut ped = Ped::new();
        if generations == 0 {
            return Ok(ped);
        }
        ped.add(Individual::founder("1", Sex::Male))?;
        if generations == 1 {
            return Ok(ped);
        }
        let mut prev_child = "1".to_string();
        let mut curr_id = 2;
        for _gen in 2..=generations {
            let spouse_id = curr_id.to_string();
            curr_id += 1;
            let child_id = curr_id.to_string();
            curr_id += 1;

            ped.add(Individual::founder(&spouse_id, Sex::Female))?;
            ped.add(Individual::child(
                &child_id,
                &prev_child,
                &spouse_id,
                Sex::Male,
            ))?;
            prev_child = child_id;
        }
        Ok(ped)
    }

    /// Gera primos duplos (double cousins).
    pub fn double_cousins(_deg1: usize, _deg2: usize) -> Result<Self, PedError> {
        let mut ped = Ped::new();
        ped.add(Individual::founder("1", Sex::Male))?;
        ped.add(Individual::founder("2", Sex::Female))?;
        ped.add(Individual::founder("3", Sex::Male))?;
        ped.add(Individual::founder("4", Sex::Female))?;

        ped.add(Individual::child("5", "1", "2", Sex::Male))?;
        ped.add(Individual::child("6", "3", "4", Sex::Female))?;
        ped.add(Individual::child("7", "1", "2", Sex::Male))?;
        ped.add(Individual::child("8", "3", "4", Sex::Female))?;

        ped.add(Individual::child("9", "5", "6", Sex::Male))?;
        ped.add(Individual::child("10", "7", "8", Sex::Male))?;
        Ok(ped)
    }
}
