use crate::Result;
use crate::{
    actor::{extract_info_param, InfoSource},
    prelude::*,
};
use join_str::jstr;
use roead::aamp::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneralParamList(pub ParameterIO);

impl From<&ParameterIO> for GeneralParamList {
    fn from(pio: &ParameterIO) -> Self {
        Self(pio.clone())
    }
}

impl From<ParameterIO> for GeneralParamList {
    fn from(pio: ParameterIO) -> Self {
        Self(pio)
    }
}

impl From<GeneralParamList> for ParameterIO {
    fn from(val: GeneralParamList) -> Self {
        val.0
    }
}

impl_simple_aamp!(GeneralParamList, 0);

impl InfoSource for GeneralParamList {
    fn update_info(&self, info: &mut roead::byml::Hash) -> crate::Result<()> {
        if let Some(obj) = self.0.object("AnimalUnit") {
            crate::actor::info_params!(obj, info, {
                ("animalUnitBasePlayRate", "BasePlayRate", f32),
            });
        };
        if let Some(obj) = self.0.object("Armor") {
            crate::actor::info_params!(obj, info, {
                ("armorDefenceAddLevel", "DefenceAddLevel", i32),
                ("armorNextRankName", "NextRankName", String),
                ("armorStarNum", "StarNum", i32),
            });
        };
        if let Some(obj) = self.0.object("ArmorEffect") {
            crate::actor::info_params!(obj, info, {
                ("armorEffectAncientPowUp", "AncientPowUp", bool),
                ("armorEffectEffectLevel", "EffectLevel", i32),
                ("armorEffectEffectType", "EffectType", String),
                ("armorEffectEnableClimbWaterfall", "EnableClimbWaterfall", bool),
                ("armorEffectEnableSpinAttack", "EnableSpinAttack", bool),
            });
        };
        if let Some(obj) = self.0.object("ArmorHead") {
            crate::actor::info_params!(obj, info, {
                ("armorHeadMantleType", "HeadMantleType", i32)
            });
        };
        if let Some(obj) = self.0.object("ArmorUpper") {
            crate::actor::info_params!(obj, info, {
                ("armorUpperDisableSelfMantle", "DisableSelfMantle", bool),
                ("armorUpperUseMantleType", "UseMantleType", i32),
            });
        };
        if let Some(obj) = self.0.object("Arrow") {
            crate::actor::info_params!(obj, info, {
                ("arrowArrowDeletePer", "ArrowDeletePer", i32),
                ("arrowArrowNum", "ArrowNum", i32),
                ("arrowDeleteTime", "DeleteTime", i32),
                ("arrowDeleteTimeWithChemical", "DeleteTimeWithChemical", i32),
                ("arrowEnemyShootNumForDelete", "EnemyShootNumForDelete", i32),
            });
        };
        if let Some(obj) = self.0.object("Attack") {
            crate::actor::info_params!(obj, info, { ("attackPower", "Power", i32) });
        };
        if let Some(obj) = self.0.object("Bow") {
            crate::actor::info_params!(obj, info, {
                ("bowArrowName", "ArrowName", String),
                ("bowIsLeadShot", "IsLeadShot", bool),
                ("bowIsRapidFire", "IsRapidFire", bool),
                ("bowLeadShotNum", "LeadShotNum", i32),
                ("bowRapidFireNum", "RapidFireNum", i32),
            });
        };
        if let Some(obj) = self.0.object("CookSpice") {
            crate::actor::info_params!(obj, info, {
                ("cookSpiceBoostEffectiveTime", "BoostEffectiveTime", i32),
                ("cookSpiceBoostHitPointRecover", "BoostHitPointRecover", i32),
                ("cookSpiceBoostMaxHeartLevel", "BoostMaxHeartLevel", i32),
                ("cookSpiceBoostStaminaLevel", "BoostStaminaLevel", i32),
                ("cookSpiceBoostSuccessRate", "BoostSuccessRate", i32),
            });
        };
        if let Some(obj) = self.0.object("CureItem") {
            crate::actor::info_params!(obj, info, {
                ("cookSpiceBoostEffectiveTime", "BoostEffectiveTime", i32),
                ("cookSpiceBoostHitPointRecover", "BoostHitPointRecover", i32),
                ("cookSpiceBoostMaxHeartLevel", "BoostMaxHeartLevel", i32),
                ("cookSpiceBoostStaminaLevel", "BoostStaminaLevel", i32),
                ("cookSpiceBoostSuccessRate", "BoostSuccessRate", i32),
            });
        };
        if let Some(obj) = self.0.object("Enemy") {
            crate::actor::info_params!(obj, info, {
                ("enemyRank", "Rank", i32),
            });
        };
        if let Some(obj) = self.0.object("General") {
            crate::actor::info_params!(obj, info, {
                ("generalLife", "Life", i32),
            });
        };
        if let Some(obj) = self.0.object("Horse") {
            crate::actor::info_params!(obj, info, {
                ("horseASVariation", "ASVariation", String),
                ("horseGearTopChargeNum", "GearTopChargeNum", i32),
                ("horseNature", "Nature", i32),
            });
        };
        if let Some(obj) = self.0.object("HorseUnit") {
            crate::actor::info_params!(obj, info, {
                ("horseUnitRiddenAnimalType", "RiddenAnimalType", i32),
            });
        };
        if let Some(obj) = self.0.object("Item") {
            crate::actor::info_params!(obj, info, {
                ("itemBuyingPrice", "BuyingPrice", i32),
                ("itemCreatingPrice", "CreatingPrice", i32),
                ("itemSaleRevivalCount", "SaleRevivalCount", i32),
                ("itemSellingPrice", "SellingPrice", i32),
                ("itemStainColor", "StainColor", i32),
                ("itemUseIconActorName", "UseIconActorName", String),
            });
        };
        if let Some(obj) = self.0.object("MasterSword") {
            crate::actor::info_params!(obj, info, {
                ("masterSwordSearchEvilDist", "SearchEvilDist", f32),
                ("masterSwordSleepActorName", "SleepActorName", String),
                ("masterSwordTrueFormActorName", "TrueFormActorName", String),
                ("masterSwordTrueFormAttackPower", "TrueFormAttackPower", i32),
            });
        };
        if let Some(obj) = self.0.object("MonsterShop") {
            crate::actor::info_params!(obj, info, {
                ("monsterShopBuyMamo", "BuyMamo", i32),
                ("monsterShopSellMamo", "SellMamo", i32),
            });
        };
        if let Some(obj) = self.0.object("PictureBook") {
            crate::actor::info_params!(obj, info, {
                ("pictureBookLiveSpot1", "LiveSpot1", i32),
                ("pictureBookLiveSpot2", "LiveSpot2", i32),
                ("pictureBookSpecialDrop", "SpecialDrop", i32),
            });
        };
        if let Some(obj) = self.0.object("Rupee") {
            crate::actor::info_params!(obj, info, {
                ("rupeeRupeeValue", "RupeeValue", i32),
            });
        };
        if let Some(obj) = self.0.object("SeriesArmor") {
            crate::actor::info_params!(obj, info, {
                ("seriesArmorEnableCompBonus", "EnableCompBonus", bool),
                ("seriesArmorSeriesType", "SeriesType", String),
            });
        };
        if let Some(obj) = self.0.object("System") {
            crate::actor::info_params!(obj, info, {
                ("systemIsGetItemSelf", "IsGetItemSelf", bool),
                ("systemSameGroupActorName", "SameGroupActorName", String),
            });
        };
        if let Some(obj) = self.0.object("Traveler") {
            [
                "AppearGameDataName".to_owned(),
                "DeleteGameDataName".to_owned(),
                "RideHorseName".to_owned(),
                "RouteType".to_owned(),
            ]
            .into_iter()
            .chain((0..30).map(|i| format!("RoutePoint{}Name", i)))
            .try_for_each(|param| -> Result<()> {
                if let Some(val) = extract_info_param::<String>(obj, &param)?
                    && val.as_string().map(|v| !v.is_empty()).unwrap_or_default()
                {
                    info.insert(jstr!("traveler{&param}"), val);
                }
                Ok(())
            })?;
        };
        if let Some(obj) = self.0.object("WeaponCommon") {
            crate::actor::info_params!(obj, info, {
                ("weaponCommonGuardPower", "GuardPower", i32),
                ("weaponCommonPoweredSharpAddAtkMax", "PoweredSharpAddAtkMax", i32),
                ("weaponCommonPoweredSharpAddAtkMin", "PoweredSharpAddAtkMin", i32),
                ("weaponCommonPoweredSharpAddLifeMax", "PoweredSharpAddLifeMax", i32),
                ("weaponCommonPoweredSharpAddLifeMin", "PoweredSharpAddLifeMin", i32),
                (
                    "weaponCommonPoweredSharpAddRapidFireMax",
                    "PoweredSharpAddRapidFireMax",
                    f32
                ),
                (
                    "weaponCommonPoweredSharpAddRapidFireMin",
                    "PoweredSharpAddRapidFireMin",
                    f32
                ),
                ("weaponCommonPoweredSharpAddSpreadFire", "PoweredSharpAddSpreadFire", bool),
                ("weaponCommonPoweredSharpAddSurfMaster", "PoweredSharpAddSurfMaster", bool),
                ("weaponCommonPoweredSharpAddThrowMax", "PoweredSharpAddThrowMax", f32),
                ("weaponCommonPoweredSharpAddThrowMin", "PoweredSharpAddThrowMin", f32),
                ("weaponCommonPoweredSharpAddZoomRapid", "PoweredSharpAddZoomRapid", bool),
                (
                    "weaponCommonPoweredSharpWeaponAddGuardMax",
                    "PoweredSharpWeaponAddGuardMax",
                    i32
                ),
                (
                    "weaponCommonPoweredSharpWeaponAddGuardMin",
                    "PoweredSharpWeaponAddGuardMin",
                    i32
                ),
                ("weaponCommonRank", "Rank", i32),
                ("weaponCommonSharpWeaponAddAtkMax", "SharpWeaponAddAtkMax", i32),
                ("weaponCommonSharpWeaponAddAtkMin", "SharpWeaponAddAtkMin", i32),
                ("weaponCommonSharpWeaponAddCrit", "SharpWeaponAddCrit", bool),
                ("weaponCommonSharpWeaponAddGuardMax", "SharpWeaponAddGuardMax", i32),
                ("weaponCommonSharpWeaponAddGuardMin", "SharpWeaponAddGuardMin", i32),
                ("weaponCommonSharpWeaponAddLifeMax", "SharpWeaponAddLifeMax", i32),
                ("weaponCommonSharpWeaponAddLifeMin", "SharpWeaponAddLifeMin", i32),
                ("weaponCommonSharpWeaponPer", "SharpWeaponPer", f32),
                ("weaponCommonStickDamage", "StickDamage", i32),
            });
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{actor::InfoSource, prelude::*};

    #[test]
    fn serde() {
        let actor = crate::tests::test_base_actorpack("Enemy_Guardian_A");
        let pio = roead::aamp::ParameterIO::from_binary(
            actor
                .get_file_data("Actor/GeneralParamList/Enemy_Guardian_A.bgparamlist")
                .unwrap(),
        )
        .unwrap();
        let gparamlist = super::GeneralParamList::try_from(&pio).unwrap();
        let data = gparamlist.clone().into_pio().to_binary();
        let pio2 = roead::aamp::ParameterIO::from_binary(&data).unwrap();
        let gparamlist2 = super::GeneralParamList::try_from(&pio2).unwrap();
        assert_eq!(gparamlist, gparamlist2);
    }

    #[test]
    fn diff() {
        let actor = crate::tests::test_base_actorpack("Enemy_Guardian_A");
        let pio = roead::aamp::ParameterIO::from_binary(
            actor
                .get_file_data("Actor/GeneralParamList/Enemy_Guardian_A.bgparamlist")
                .unwrap(),
        )
        .unwrap();
        let gparamlist = super::GeneralParamList::try_from(&pio).unwrap();
        let actor2 = crate::tests::test_mod_actorpack("Enemy_Guardian_A");
        let pio2 = roead::aamp::ParameterIO::from_binary(
            actor2
                .get_file_data("Actor/GeneralParamList/Enemy_Guardian_A.bgparamlist")
                .unwrap(),
        )
        .unwrap();
        let gparamlist2 = super::GeneralParamList::try_from(&pio2).unwrap();
        let diff = gparamlist.diff(&gparamlist2);
        println!("{}", serde_json::to_string_pretty(&diff).unwrap());
    }

    #[test]
    fn merge() {
        let actor = crate::tests::test_base_actorpack("Enemy_Guardian_A");
        let pio = roead::aamp::ParameterIO::from_binary(
            actor
                .get_file_data("Actor/GeneralParamList/Enemy_Guardian_A.bgparamlist")
                .unwrap(),
        )
        .unwrap();
        let actor2 = crate::tests::test_mod_actorpack("Enemy_Guardian_A");
        let gparamlist = super::GeneralParamList::try_from(&pio).unwrap();
        let pio2 = roead::aamp::ParameterIO::from_binary(
            actor2
                .get_file_data("Actor/GeneralParamList/Enemy_Guardian_A.bgparamlist")
                .unwrap(),
        )
        .unwrap();
        let gparamlist2 = super::GeneralParamList::try_from(&pio2).unwrap();
        let diff = gparamlist.diff(&gparamlist2);
        let merged = gparamlist.merge(&diff);
        assert_eq!(gparamlist2, merged);
    }

    #[test]
    fn info() {
        use roead::byml::Byml;
        let actor = crate::tests::test_mod_actorpack("Enemy_Guardian_A");
        let pio = roead::aamp::ParameterIO::from_binary(
            actor
                .get_file_data("Actor/GeneralParamList/Enemy_Guardian_A.bgparamlist")
                .unwrap(),
        )
        .unwrap();
        let gparamlist = super::GeneralParamList::try_from(&pio).unwrap();
        let mut info = roead::byml::Hash::new();
        gparamlist.update_info(&mut info).unwrap();
        assert_eq!(info["systemIsGetItemSelf"], Byml::Bool(false));
        assert_eq!(
            info["systemSameGroupActorName"],
            Byml::String("Enemy_Guardian_A_Mod".to_owned())
        );
        assert_eq!(info["generalLife"], Byml::Int(1500000));
        assert_eq!(info["enemyRank"], Byml::Int(15));
        assert_eq!(info["attackPower"], Byml::Int(0));
        assert_eq!(info["pictureBookLiveSpot1"], Byml::Int(27));
        assert_eq!(
            info["travelerAppearGameDataName"],
            Byml::String("Testing".to_owned())
        );
        assert_eq!(
            info["travelerRoutePoint24Name"],
            Byml::String("SomePoint".to_owned())
        );
    }
}
