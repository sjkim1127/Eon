import type { HanjaHangul } from "../types";

// ──────────────────────────────────────────────
// 사주 Rust enum → 한자/한글 변환 테이블
// ──────────────────────────────────────────────

export const STEM_INFO: Record<string, HanjaHangul> = {
  Jia:  { hanja: "甲", hangul: "갑" }, Yi:   { hanja: "乙", hangul: "을" },
  Bing: { hanja: "丙", hangul: "병" }, Ding: { hanja: "丁", hangul: "정" },
  Wu:   { hanja: "戊", hangul: "무" }, Ji:   { hanja: "己", hangul: "기" },
  Geng: { hanja: "庚", hangul: "경" }, Xin:  { hanja: "辛", hangul: "신" },
  Ren:  { hanja: "壬", hangul: "임" }, Gui:  { hanja: "癸", hangul: "계" },
};

export const BRANCH_INFO: Record<string, HanjaHangul> = {
  Zi:   { hanja: "子", hangul: "자" }, Chou: { hanja: "丑", hangul: "축" },
  Yin:  { hanja: "寅", hangul: "인" }, Mao:  { hanja: "卯", hangul: "묘" },
  Chen: { hanja: "辰", hangul: "진" }, Si:   { hanja: "巳", hangul: "사" },
  Wu:   { hanja: "午", hangul: "오" }, Wei:  { hanja: "未", hangul: "미" },
  Shen: { hanja: "申", hangul: "신" }, You:  { hanja: "酉", hangul: "유" },
  Xu:   { hanja: "戌", hangul: "술" }, Hai:  { hanja: "亥", hangul: "해" },
};

export const ELEMENT_INFO: Record<string, HanjaHangul> = {
  Wood:  { hanja: "木", hangul: "목" }, Fire:  { hanja: "火", hangul: "화" },
  Earth: { hanja: "土", hangul: "토" }, Metal: { hanja: "金", hangul: "금" },
  Water: { hanja: "水", hangul: "수" },
};

export const STRENGTH_INFO: Record<string, string> = {
  Strong: "신강 (身强)", Weak: "신약 (身弱)", Balanced: "중화 (中和)",
};

export const TENGOD_INFO: Record<string, HanjaHangul> = {
  Bijian:    { hangul: "비견", hanja: "比肩" }, Jiecai:    { hangul: "겁재", hanja: "劫財" },
  Shishen:   { hangul: "식신", hanja: "食神" }, Shangguan: { hangul: "상관", hanja: "傷官" },
  Piancai:   { hangul: "편재", hanja: "偏財" }, Zhengcai:  { hangul: "정재", hanja: "正財" },
  Pianguan:  { hangul: "편관", hanja: "偏官" }, Zhengguan: { hangul: "정관", hanja: "正官" },
  Pianyin:   { hangul: "편인", hanja: "偏印" }, Zhengyin:  { hangul: "정인", hanja: "正印" },
};

export const STRUCTURE_INFO: Record<string, HanjaHangul> = {
  ShiShen: { hangul: "식신격", hanja: "食神格" }, ShangGuan: { hangul: "상관격", hanja: "傷官格" },
  PianCai:  { hangul: "편재격", hanja: "偏財格" }, ZhengCai:  { hangul: "정재격", hanja: "正財格" },
  PianGuan: { hangul: "편관격", hanja: "偏官格" }, ZhengGuan: { hangul: "정관격", hanja: "正官格" },
  PianYin:  { hangul: "편인격", hanja: "偏印格" }, ZhengYin:  { hangul: "정인격", hanja: "正印格" },
  JianLu:   { hangul: "건록격", hanja: "建祿格" }, YangIn:    { hangul: "양인격", hanja: "陽刃格" },
  Special:  { hangul: "비겁격", hanja: "特殊格" },
  JongAh:   { hangul: "종아격", hanja: "從兒格" }, JongJae:   { hangul: "종재격", hanja: "從財格" },
  JongSal:  { hangul: "종살격", hanja: "從殺格" }, JongGang:  { hangul: "종강격", hanja: "從强格" },
  JongWang: { hangul: "종왕격", hanja: "從旺格" }, Follower:  { hangul: "종격", hanja: "從格" },
  SpecialTransformation: { hangul: "전왕격", hanja: "專旺格" },
};

export const SPIRIT_INFO: Record<string, HanjaHangul> = {
  Tianyi: { hangul: "천을귀인", hanja: "天乙貴人" }, Wenchang: { hangul: "문창귀인", hanja: "文昌貴人" },
  Taiji: { hangul: "태극귀인", hanja: "太極貴人" }, Yuede: { hangul: "월덕귀인", hanja: "月德貴人" },
  Tiande: { hangul: "천덕귀인", hanja: "天德貴人" }, Zhenglu: { hangul: "정록", hanja: "正祿" },
  Jinyu: { hangul: "금여록", hanja: "金輿祿" }, Anlu: { hangul: "암록", hanja: "暗祿" },
  Xuetang: { hangul: "학당귀인", hanja: "學堂貴人" }, TianyiMedical: { hangul: "천의성", hanja: "天醫星" },
  Tianwen: { hangul: "천문성", hanja: "天文星" }, Yima: { hangul: "역마살", hanja: "驛馬煞" },
  Huagai: { hangul: "화개살", hanja: "華蓋煞" }, Kuigang: { hangul: "괴강살", hanja: "魁罡煞" },
  Taohua: { hangul: "도화살", hanja: "桃花煞" }, Hongyan: { hangul: "홍염살", hanja: "紅艶煞" },
  Guchen: { hangul: "고신살", hanja: "孤辰煞" }, Guasu: { hangul: "과숙살", hanja: "寡宿煞" },
  Xuanzhen: { hangul: "현침살", hanja: "懸針煞" }, Baihu: { hangul: "백호살", hanja: "白虎煞" },
  Wangshen: { hangul: "망신살", hanja: "亡身煞" }, Jiesha: { hangul: "겁살", hanja: "劫煞" },
  Yuanzhen: { hangul: "원진살", hanja: "怨嗔煞" }, Jaesha: { hangul: "재살", hanja: "災煞" },
  Cheonsha: { hangul: "천살", hanja: "天煞" }, Jisha: { hangul: "지살", hanja: "地煞" },
  Nyeonsha: { hangul: "년살", hanja: "年煞" }, Wolsha: { hangul: "월살", hanja: "月煞" },
  Jangseong: { hangul: "장성살", hanja: "將星煞" }, Banan: { hangul: "반안살", hanja: "潘鞍煞" },
  Yukhae: { hangul: "육해살", hanja: "六害煞" },
};

export const PILLAR_POS_INFO: Record<string, string> = {
  Year: "년주", Month: "월주", Day: "일주", Hour: "시주",
};

export const YONGSHIN_TYPE_INFO: Record<string, string> = {
  Eokbu: "억부용신",
  Johu: "조후용신",
  Tonggwan: "통관용신",
  Byeongyak: "병약용신",
};
