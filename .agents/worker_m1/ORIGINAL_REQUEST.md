## 2026-07-25T03:20:00Z
You are the Implementation Worker for Milestone 1 (R1): Core Analysis Precision & Pattern Completeness in `crates/eon-saju`.

Read `AGENTS.md` at `/Users/sjkim1127/Eon/AGENTS.md` first.
Read the Explorer handoff reports at:
- `/Users/sjkim1127/Eon/.agents/teamwork_preview_explorer_m1_1/handoff.md`
- `/Users/sjkim1127/Eon/.agents/teamwork_preview_explorer_m1_2/handoff.md`
- `/Users/sjkim1127/Eon/.agents/teamwork_preview_explorer_m1_3/handoff.md`

Your tasks for Milestone 1 (R1):
1. **Eokbu(抑扶) Yongsin & Power Score Refactoring (`crates/eon-saju/src/analysis/yongshin.rs`)**:
   - Refactor Eokbu calculation to use integrated weighted power percentages from `power.rs` (`integrated.ten_god_scores`) instead of raw integer counts (`yinxing_count`/`bijie_count`).
   - Fix weak Day Master selection: If weak DM is caused by excessive Caisheng (Weak, Caisheng > 35%), select **BiGeop (BiGeop)** as Eokbu Yongsin when Yinxing is weak/absent. Select **Inseong (Resource)** when weak DM is caused by GuanXing or ShiShang overload.
2. **Unified Primary Yongsin Priority Scoring Algorithm**:
   - Add Tonggwan (TongGwanYongshin) into primary Yongsin selection (`primary`) when severe 50/50 elemental conflict is present. Implement a multi-factorial priority scoring matrix so urgent clashes or thermal extremes take primary status when appropriate.
3. **ByeongYak(病藥) Yongsin Diagnostic Structure**:
   - Add explicit `Byeong` (Disease Element) and `Yak` (Medicine Element) tracking structures. Identify Disease when any single element exceeds 40% power or when Yongsin is directly clashed, selecting Medicine as the controlling/mediating element.
4. **JoHu(調候) Yongsin Stem Specification**:
   - Enhance Johu recommendation to specify preferred Heavenly Stems (*Qiong Tong Bao Giam* /  JoHu preferences, e.g. 丙火 vs 丁火 in winter; 癸水 vs 壬水 in summer).
5. **Special Gyeokguk Refinement (`crates/eon-saju/src/analysis/structure.rs`)**:
   - Add Jin-Jong (眞從) vs Ga-Jong (假從) root checks: verify hidden stem roots (`root_score`) of DM/Yinxing before classifying true Jong-Gyeong.
   - Add HwaGi () break-star (破星) validation.
   - Formally classify Samhap Jeonwang outer patterns (`GokJik`, `YeomSang`, `GaSaek`, `JongHyeok`, `YoonHa`) and `GwanSalHonJab` ().
6. **12-Unseong & Sinsal Edge Cases**:
   - Add Samjae engine (`calculate_samjae`: Entering Samjae, Staying Samjae, Exiting Samjae) in `spirit_markers.rs` / `shinsal.rs` based on Year Branch triads.
   - Implement Gongmang dissolution (`Void`/`Void`) in `void.rs` & `spirit_markers.rs` when voided branch is clashed or combined.
   - Add configurable 12-Unseong Yin-Stem progression option (`yin_stem_reverse: bool`, default true) in `AnalysisConfig`.
