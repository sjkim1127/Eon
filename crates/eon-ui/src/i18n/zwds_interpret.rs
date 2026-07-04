use crate::i18n::Locale;
use eon_zwds::types::{DestinyPattern, PalaceName, SiHuaType, StarInPalace, ZwdsStar};

/// 자미두수 특정 궁위의 고급 종합 리딩 텍스트를 생성합니다.
pub fn get_advanced_palace_interpretation(
    locale: Locale,
    palace_name: PalaceName,
    stars: &[StarInPalace],
    destiny_patterns: &[DestinyPattern],
) -> String {
    let mut paragraphs = Vec::new();

    // 1. 궁위의 기본 역할 정보 추가
    let base_desc = match locale {
        Locale::Ko => match palace_name {
            PalaceName::Ming => "명궁은 자아, 성격, 평생의 운명적 지향점과 선천적인 복덕을 상징합니다. 인생 전반의 방향성을 좌우하는 가장 중요한 궁입니다.",
            PalaceName::Xiongdi => "형제궁은 형제자매, 동료와의 관계, 협력 관계 및 재정적 동반자를 의미합니다.",
            PalaceName::Fuqi => "부처궁은 배우자와의 관계, 이상적인 배우자 상, 결혼 생활의 길흉을 의미합니다.",
            PalaceName::Zinv => "자녀궁은 자녀와의 유대감, 자손의 번창 여부, 창작 및 투자 기운을 나타냅니다.",
            PalaceName::Caibo => "재백궁은 재물을 버는 방식, 수입의 원천, 재정적 능력 및 소비 성향을 상징합니다.",
            PalaceName::Jie => "질액궁은 선천적인 건강 상태, 체질, 주의해야 할 질병 및 재난을 의미합니다.",
            PalaceName::Qianyi => "천이궁은 사회 활동, 타향/해외 이동, 대인관계에서의 외적인 모습과 여행 운을 나타냅니다.",
            PalaceName::Nupao => "노복궁은 부하 직원, 친구, 지인, 사회적 인맥과의 관계 및 인복을 의미합니다.",
            PalaceName::Guanlu => "관록궁은 직업적인 성취, 학업, 승진, 적합한 직업 분야 및 사회적 위상을 상징합니다.",
            PalaceName::Tianzhai => "전택궁은 부동산, 주거 환경, 가정의 평화 및 자산 축적 형태를 의미합니다.",
            PalaceName::Fude => "복덕궁은 정신세계, 취미, 영적인 성향, 내면의 행복감 및 노후의 편안함을 상징합니다.",
            PalaceName::Fumu => "부모궁은 부모님과의 관계, 윗사람이나 국가 기관의 덕, 학문 및 문서를 상징합니다.",
        },
        Locale::Zh => match palace_name {
            PalaceName::Ming => "命宫代表自我、性格、一生的命运走向和先天福德。是决定人生大方向的最核心宫位。",
            PalaceName::Xiongdi => "兄弟宫代表与兄弟姐妹、亲密同事的关系，以及财务上的合作伙伴。",
            PalaceName::Fuqi => "夫妻宫掌管婚姻、恋爱风格、理想伴侣的特征以及婚姻生活的吉凶。",
            PalaceName::Zinv => "子女宫代表与子女的缘分、子嗣昌盛与否、创造力以及小额投资运气。",
            PalaceName::Caibo => "财帛宫描述赚钱能力、收入来源、财务理财方式和消费倾向。",
            PalaceName::Jie => "疾厄宫代表先天健康状况、体质、容易罹患的疾病和人生灾难。",
            PalaceName::Qianyi => "迁移宫掌管社会活动、出外及海外发展、人际关系中的外在形象和旅行运势。",
            PalaceName::Nupao => "奴仆宫代表与下属、朋友、人脉圈的关系以及社交福气。",
            PalaceName::Guanlu => "官禄宫代表职业成就、学业运势、升迁、适合 the 行业及社会地位。",
            PalaceName::Tianzhai => "田宅宫代表不动产运势、居住环境、家业继承、家庭和谐度以及资产累积形态。",
            PalaceName::Fude => "福德宫代表精神世界、兴趣爱好、潜意识、内心幸福感以及晚年享福运。",
            PalaceName::Fumu => "父母宫代表与父母的关系、长辈及上司的提携、学业证书和法律契约。",
        },
        Locale::Ru => match palace_name {
            PalaceName::Ming => "Дворец Жизни представляет собой эго, личность, жизненный путь и врожденную карму. Самый важный дворец.",
            PalaceName::Xiongdi => "Дворец Братьев представляет отношения с братьями и сестрами, близкими коллегами и партнерами.",
            PalaceName::Fuqi => "Дворец Супружества управляет браком, стилем отношений и чертами вашего идеального партнера.",
            PalaceName::Zinv => "Дворец Детей представляет отношения с детьми, продолжение рода, творчество и мелкие инвестиции.",
            PalaceName::Caibo => "Дворец Богатства описывает вашу способность зарабатывать, источники дохода и стиль трат.",
            PalaceName::Jie => "Дворец Здоровья указывает на врожденную конституцию тела, возможные болезни и самочувствие.",
            PalaceName::Qianyi => "Дворец Перемещений представляет социальную активность, переезды, поездки и внешний имидж.",
            PalaceName::Nupao => "Дворец Друзей управляет отношениями с подчиненными, друзьями, коллегами и связями.",
            PalaceName::Guanlu => "Дворец Карьеры представляет профессиональные достижения, учебу, пригодность к работе и статус.",
            PalaceName::Tianzhai => "Дворец Имущества управляет недвижимостью, домом, семейным наследием и накоплениями.",
            PalaceName::Fude => "Дворец Кармы (Счастья) представляет духовный мир, хобби, счастье и комфорт в старости.",
            PalaceName::Fumu => "Дворец Родителей представляет отношения с родителями, покровителями, учебу и документы.",
        },
        Locale::En => match palace_name {
            PalaceName::Ming => "The Life Palace represents the self, personality, lifelong destiny, and innate fortune. It is the most critical palace governing your life.",
            PalaceName::Xiongdi => "The Siblings Palace represents relationships with siblings, close colleagues, and financial partners.",
            PalaceName::Fuqi => "The Spouse Palace governs marriage, relationship style, and the traits of your ideal partner.",
            PalaceName::Zinv => "The Children Palace represents relationships with children, descendant luck, creativity, and minor investments.",
            PalaceName::Caibo => "The Wealth Palace describes your earning capacity, income sources, and financial management style.",
            PalaceName::Jie => "The Health Palace indicates physical constitution, potential diseases, and general physical well-being.",
            PalaceName::Qianyi => "The Travel Palace represents social relations, relocation, travel fortunes, and how you appear to the public.",
            PalaceName::Nupao => "The Friends Palace governs relationships with subordinates, friends, social circles, and general networking luck.",
            PalaceName::Guanlu => "The Career Palace represents professional achievements, academic success, job suitability, and public status.",
            PalaceName::Tianzhai => "The Property Palace governs real estate, home environment, family inheritance, and long-term asset accumulation.",
            PalaceName::Fude => "The Karma Palace represents your inner mind, spiritual affinity, hobbies, happiness, and comfort in late life.",
            PalaceName::Fumu => "The Parents Palace represents relations with mentors or government, education, and legal documents.",
        }
    };
    paragraphs.push(base_desc.to_string());

    // 2. 주요 동궁(Dual-star) 조합 감지 및 해석 추가
    let has_star = |target: ZwdsStar| stars.iter().any(|s| s.star == target);

    let comb_desc = match locale {
        Locale::Ko => {
            if has_star(ZwdsStar::ZiWei) && has_star(ZwdsStar::TianFu) {
                Some("★ 자부동궁(紫府同宮): 자미와 천부가 함께 자리하여 자산 보존력과 권위가 극대화됩니다. 곳간이 든든한 황제처럼 매사 안정적이고 주도적인 흐름을 만들어 냅니다.")
            } else if has_star(ZwdsStar::LianZhen) && has_star(ZwdsStar::TanLang) {
                Some("★ 염탐동궁(廉貪同宮): 감성과 욕망의 별들이 만나 강렬한 사교성과 예술적 매력, 도화 기운이 발현됩니다. 대인관계에서 인기가 많으나 감정 조절에 유의하십시오.")
            } else if has_star(ZwdsStar::WuQu) && has_star(ZwdsStar::TanLang) {
                Some("★ 무탐격(武貪格): 무곡과 탐랑이 동궁하여 젊은 날의 고생 끝에 중년 이후 급격한 금전적 자수성가를 이룩하는 대기만성형 흐름을 유발합니다.")
            } else if has_star(ZwdsStar::ZiWei) && has_star(ZwdsStar::QiSha) {
                Some("★ 자살동궁(紫殺同宮): 제왕의 지위와 장수의 권위가 결합되어 강한 추진력과 창조적 파괴를 의미합니다. 기존 질서를 혁신하고 선구자가 될 수 있습니다.")
            } else if has_star(ZwdsStar::WuQu) && has_star(ZwdsStar::QiSha) {
                Some("★ 무살동궁(武殺同宮): 결단력과 과감한 기술적 추진력이 결합됩니다. 이공계, 기술직군, 혹은 과감한 결단이 요구되는 무역/금융업에 매우 강한 성향을 보입니다.")
            } else {
                None
            }
        }
        Locale::Zh => {
            if has_star(ZwdsStar::ZiWei) && has_star(ZwdsStar::TianFu) {
                Some("★ 紫府同宫：紫微与天府同守此宫，财富蓄积力与权威极大化。如同府库充裕的帝王，万事稳定且掌管主动权。")
            } else if has_star(ZwdsStar::LianZhen) && has_star(ZwdsStar::TanLang) {
                Some("★ 廉贪同宫：情感与欲望之星相遇，展现强烈的社交与艺术魅力，桃花极旺。人缘极佳，但需注意情绪自控。")
            } else if has_star(ZwdsStar::WuQu) && has_star(ZwdsStar::TanLang) {
                Some("★ 武贪格：武曲与贪狼同守，主青壮年历经磨练，中年（30岁后）一举白手起家，为大器晚成之财气爆发组合。")
            } else if has_star(ZwdsStar::ZiWei) && has_star(ZwdsStar::QiSha) {
                Some("★ 紫杀同宫：帝星与将星联手，主强大执行力与创造性突破。敢于打破陈规，担当行业领头羊。")
            } else if has_star(ZwdsStar::WuQu) && has_star(ZwdsStar::QiSha) {
                Some("★ 武杀同宫：决断力与刚毅的技术冲劲交织。适合理工、技术、军警，或需要杀伐决断的商贸金融行业。")
            } else {
                None
            }
        }
        Locale::Ru => {
            if has_star(ZwdsStar::ZiWei) && has_star(ZwdsStar::TianFu) {
                Some("★ Цзы-Фу: Совместное нахождение Цзы Вэй и Тянь Фу объединяет авторитет и финансовую стабильность. Подобно богатому императору, вы держите бразды правления.")
            } else if has_star(ZwdsStar::LianZhen) && has_star(ZwdsStar::TanLang) {
                Some("★ Лянь-Тань: Встреча чувственности и страсти дает высокое обаяние, популярность и склонность к искусству. Остерегайтесь искушений.")
            } else if has_star(ZwdsStar::WuQu) && has_star(ZwdsStar::TanLang) {
                Some("★ У-Тань: Соединение У Цюй и Тань Лан. Трудная молодость окупается ошеломительным богатством и успехом в зрелые годы.")
            } else if has_star(ZwdsStar::ZiWei) && has_star(ZwdsStar::QiSha) {
                Some("★ Цзы-Ша: Союз Императора и Генерала. Ведет к решительным прорывам, независимости и реформированию старых порядков.")
            } else if has_star(ZwdsStar::WuQu) && has_star(ZwdsStar::QiSha) {
                Some("★ У-Ша: Высокая техническая хватка и сила воли. Подходит для инженерии, финансов и сфер, требующих быстрых решений.")
            } else {
                None
            }
        }
        Locale::En => {
            if has_star(ZwdsStar::ZiWei) && has_star(ZwdsStar::TianFu) {
                Some("★ Zi-Fu Combination: Zi Wei and Tian Fu reside together, maximizing authority and financial storage. Like an Emperor with a full treasury, you govern stably.")
            } else if has_star(ZwdsStar::LianZhen) && has_star(ZwdsStar::TanLang) {
                Some("★ Lian-Tan Combination: Emotions and desires meet, bringing high social magnetics, artistic charms, and romance. Watch out for emotional tempests.")
            } else if has_star(ZwdsStar::WuQu) && has_star(ZwdsStar::TanLang) {
                Some("★ Wu-Tan Pattern: Wu Qu and Tan Lang meet. Although early years may bring hardship, you are a self-made winner achieving great fortune after midlife.")
            } else if has_star(ZwdsStar::ZiWei) && has_star(ZwdsStar::QiSha) {
                Some("★ Zi-Sha Combination: Emperor and General join forces, representing intense drive and creative destruction. You are born to reform and lead.")
            } else if has_star(ZwdsStar::WuQu) && has_star(ZwdsStar::QiSha) {
                Some("★ Wu-Sha Combination: Decisiveness and hard technical drive meet. Excellent for engineering, finance, or fields demanding swift judgments.")
            } else {
                None
            }
        }
    };
    if let Some(desc) = comb_desc {
        paragraphs.push(desc.to_string());
    }

    // 3. 주요 단독 주성별 배치 상세 분석 (가장 핵심적인 궁위 매칭)
    for star_in_p in stars {
        let star = star_in_p.star;
        let star_desc = match locale {
            Locale::Ko => match (star, palace_name) {
                (ZwdsStar::ZiWei, PalaceName::Ming) => Some("• 자미(명궁): 제왕의 풍모를 지녀 리더십과 자존심이 매우 강합니다. 품위와 체면을 귀하게 여기며, 귀인의 인덕을 입으나 때로 홀로 고고하여 느끼는 고독감을 관리하는 것이 중요합니다."),
                (ZwdsStar::ZiWei, PalaceName::Caibo) => Some("• 자미(재백궁): 규모가 크고 지위 높은 곳에서 발생하는 고귀한 재물을 취합니다. 주로 명예와 리더십을 활용해 부를 쌓으나, 품위 유지를 위한 씀씀이를 경계해야 합니다."),
                (ZwdsStar::ZiWei, PalaceName::Fuqi) => Some("• 자미(부처궁): 배우자가 자존심이 세고 높은 자의식을 지닌 경우가 많습니다. 리더십이 있는 동반자이나 서로 주도권 다툼을 조심해야 가정이 평안합니다."),
                (ZwdsStar::ZiWei, PalaceName::Guanlu) => Some("• 자미(관록궁): 리더가 될 직업적 운명을 지닙니다. 기관이나 기업에서 관리직, 독립적인 사업가, 혹은 높은 사회적 명예를 획득하는 직군에 잘 맞습니다."),

                (ZwdsStar::TianJi, PalaceName::Ming) => Some("• 천기(명궁): 총명하고 지혜로우며 분석력과 기획력이 매우 뛰어납니다. 새로운 지식 습득이 빠르나 과도한 근심과 생각으로 예민해지기 쉬우니 마인드 컨트롤이 요망됩니다."),
                (ZwdsStar::TianJi, PalaceName::Caibo) => Some("• 천기(재백궁): 번뜩이는 기획력, 계산, 지적 재산 또는 유통과 변화가 잦은 업종을 통해 재물을 법니다. 고정적이고 지루한 재정 운용보다는 머리를 활용한 동적 재무에 유리합니다."),
                (ZwdsStar::TianJi, PalaceName::Fuqi) => Some("• 천기(부처궁): 영리하고 말재주 있는 배우자와 인연이 깊습니다. 때로 부부 사이에 잦은 이동이나 의견 불일치로 인한 감정 변동을 유의하십시오."),
                (ZwdsStar::TianJi, PalaceName::Guanlu) => Some("• 천기(관록궁): 설계, IT, 연구, 분석, 기획 등 머리와 전문 기술을 요하는 업무에 대단히 우수합니다. 프리랜서나 잦은 이동이 따르는 직군에 적합합니다."),

                (ZwdsStar::TaiYang, PalaceName::Ming) => Some("• 태양(명궁): 열정적이고 외향적이며 공익과 타인을 돕는 일에 헌신합니다. 사회적 명예를 중시하지만, 주변을 챙기느라 정작 자신은 고달파지기 쉽습니다."),
                (ZwdsStar::TaiYang, PalaceName::Caibo) => Some("• 태양(재백궁): 널리 밝히는 기질로 인해 명예를 통해 재물을 창출합니다. 사적인 욕심보다 공적인 가치나 대중 서비스적인 성격의 비즈니스에서 정당한 대가를 얻습니다."),

                (ZwdsStar::WuQu, PalaceName::Caibo) => Some("• 무곡(재백궁): 최고의 재성(財星)이 제자리에 위치했습니다. 추진력과 단호한 실행력으로 실질적인 자산을 틀어쥐며, 금융이나 기술적 제조 등 실재하는 수단을 통해 확실히 부를 다집니다."),
                (ZwdsStar::WuQu, PalaceName::Ming) => Some("• 무곡(명궁): 성격이 곧고 결단력이 강하며 실리적입니다. 다소 차가운 느낌을 주거나 고독한 성향이 있을 수 있으니 인간관계의 유연성을 기르면 매사 길합니다."),

                (ZwdsStar::TianTong, PalaceName::Ming) => Some("• 천동(명궁): 온화하고 순박하며 감성적입니다. 주변 사람들에게 편안함과 조화를 주며 일생 의식의 곤란이 적은 복성이나, 의지력 부족이나 게으름에 빠지지 않도록 정진해야 합니다."),

                (ZwdsStar::LianZhen, PalaceName::Ming) => Some("• 염정(명궁): 주관이 아주 뚜렷하고 예술성이나 직관력이 뛰어납니다. 내면의 집념과 고집이 대단하며, 사교적인 도화 기운이 잘 조화될 때 큰 사회적 성공을 이룹니다."),

                (ZwdsStar::TianFu, PalaceName::Tianzhai) => Some("• 천부(전택궁): 하늘의 곳간이 전택궁에 놓여 부동산 복과 가정의 안전성이 확립됩니다. 저축과 실질적 건물 확보를 통해 자산을 가장 견고하게 지켜내는 흐름입니다."),

                (ZwdsStar::TaiYin, PalaceName::Caibo) => Some("• 태음(재백궁): 조용히 흐르는 강물처럼 지속적인 수입과 계획적인 자산 축적에 최적화됩니다. 부동산이나 계획적인 금융 저축을 통해 대단히 영리하게 돈을 모읍니다."),

                (ZwdsStar::TanLang, PalaceName::Ming) => Some("• 탐랑(명궁): 다재다능하고 호기심이 많으며 욕망과 사교성을 대변합니다. 도화 기운으로 인간관계의 주역이 되기 쉬우며 신비한 학문(명리, 종교)에 관심이 깊습니다."),

                (ZwdsStar::JuMen, PalaceName::Ming) => Some("• 거문(명궁): 탐구력과 분석력이 타의 추종을 불허하며 논리적인 언변을 구사합니다. 다만 불필요한 구설이나 타인과의 불신을 피하도록 차분하고 진솔한 소통 태도가 중요합니다."),

                (ZwdsStar::QiSha, PalaceName::Ming) => Some("• 칠살(명궁): 독립심과 결단력이 돋보이는 장군의 기질입니다. 강력한 돌파력으로 모험을 두려워하지 않으나, 독단으로 인한 외로움과 고독을 잘 다스려야 합니다."),

                (ZwdsStar::PoJun, PalaceName::Guanlu) => Some("• 파군(관록궁): 개척과 변화의 별이 놓여 기존의 관행을 뒤엎고 새 구조를 세우는 업무에 어울립니다. 직무에 주기적 갱신이나 독립적 창업 등이 따릅니다."),
                _ => None,
            },
            Locale::Zh => match (star, palace_name) {
                (ZwdsStar::ZiWei, PalaceName::Ming) => Some("• 紫微(命宫)：具帝王风范，领导力与自尊心极强。极重尊严与体面，得贵人相助，但需注意高处不胜寒的孤立感。"),
                (ZwdsStar::ZiWei, PalaceName::Caibo) => Some("• 紫微(财帛)：主求财于大型或高尚机构，善用名誉与地位积聚财富。消费上需控制财力流出。"),
                (ZwdsStar::ZiWei, PalaceName::Fuqi) => Some("• 紫微(夫妻)：配偶自尊心强，具自我主张。虽有担当，但日常需防范双方争夺主导权以致争执。"),
                (ZwdsStar::ZiWei, PalaceName::Guanlu) => Some("• 紫微(官禄)：事业上有做领导的潜质。适合大企业管理岗位、独立创业，或追求高社会荣誉的行业。"),

                (ZwdsStar::TianJi, PalaceName::Ming) => Some("• 天机(命宫)：聪慧过人，极擅分析与企划。吸纳新知快，但易思虑过度导致精神紧绷，宜多放松。"),
                (ZwdsStar::TianJi, PalaceName::Caibo) => Some("• 天机(财帛)：凭借企划、计算、智商或流动性行业赚取财富。利于动中求财，不适呆板理财。"),
                (ZwdsStar::TianJi, PalaceName::Fuqi) => Some("• 天机(夫妻)：易结识伶俐、善辩的伴侣。注意因时常沟通不畅或经常在外奔波引发的感情波动。"),
                (ZwdsStar::TianJi, PalaceName::Guanlu) => Some("• 天机(官禄)：极其适合设计、IT、研究、分析策划等需要高心智与脑力的专业技能岗位。"),

                (ZwdsStar::TaiYang, PalaceName::Ming) => Some("• 太阳(命宫)：充满活力与奉献心，重公益而轻私利。虽有名声，但易落得“为他人 welfare 奔波”的劳碌境地。"),
                (ZwdsStar::TaiYang, PalaceName::Caibo) => Some("• 太阳(财帛)：求财多与名声相关。适合公共事务、公众服务或名誉积累后带来正当财富回报。"),

                (ZwdsStar::WuQu, PalaceName::Caibo) => Some("• 武曲(财帛)：正财星归位。主坚决的财富进取心，执行力强，利于金融、技术开发或实业运作敛财。"),
                (ZwdsStar::WuQu, PalaceName::Ming) => Some("• 武曲(命宫)：刚直果决，求真务实。人际交往中可能略显冷漠或孤僻，宜以和为贵，放宽心态。"),

                (ZwdsStar::TianTong, PalaceName::Ming) => Some("• 天同(命宫)：温和纯真，多愁善感。主一生福报深厚，衣食不缺，但需防沉溺于安逸而丧失进取意志。"),

                (ZwdsStar::LianZhen, PalaceName::Ming) => Some("• 廉贞(命宫)：主见极强，艺术感官敏锐。心高气傲，遇吉则才华横溢，遇煞则固执任性，属于高可塑性之星。"),

                (ZwdsStar::TianFu, PalaceName::Tianzhai) => Some("• 天府(田宅)：天库落入不动产宫。主家业稳固，买房置产顺利，通过理财与房产持有最稳健储蓄财富。"),

                (ZwdsStar::TaiYin, PalaceName::Caibo) => Some("• 太阴(财帛)：财星入财位，求财稳健如细水长流。善于通过房地产或稳当储蓄，非常聪明地让钱生钱。"),

                (ZwdsStar::TanLang, PalaceName::Ming) => Some("• 贪狼(命宫)：多才多艺，欲望强烈，交游广泛。具桃花魅力，极利人际拓展，且极易对玄学宗教产生兴趣。"),

                (ZwdsStar::JuMen, PalaceName::Ming) => Some("• 巨门(命宫)：分析思辨能力极高，辞锋锐利。但在言语上需十分谨慎，避免言多必失，方能化口舌为学术。"),

                (ZwdsStar::QiSha, PalaceName::Ming) => Some("• 七杀(命宫)：具冲锋陷阵的将军风骨。独立坚强，做事果敢，但需小心因独断行事带来的落寞与阻力。"),

                (ZwdsStar::PoJun, PalaceName::Guanlu) => Some("• 破军(官禄)：开创之星居官禄，适合推陈出新、改革企业结构的工作。事业常多震荡，或适合做独立开创者。"),
                _ => None,
            },
            Locale::Ru => match (star, palace_name) {
                (ZwdsStar::ZiWei, PalaceName::Ming) => Some("• Цзы Вэй (Жизнь): Обладает качествами императора, лидерством и достоинством. Избегайте высокомерия."),
                (ZwdsStar::ZiWei, PalaceName::Caibo) => Some("• Цзы Вэй (Богатство): Накопление богатства через высокий статус или управление крупными делами."),
                (ZwdsStar::ZiWei, PalaceName::Fuqi) => Some("• Цзы Вэй (Супружество): Гордый и независимый партнер. Контролируйте стремление к доминированию."),
                (ZwdsStar::ZiWei, PalaceName::Guanlu) => Some("• Цзы Вэй (Карьера): Судьба лидера. Подходит для руководящих постов и престижных профессий."),

                (ZwdsStar::TianJi, PalaceName::Ming) => Some("• Тянь Цзи (Жизнь): Исключительный интеллект и планирование. Склонность к тревожности из-за лишних мыслей."),
                (ZwdsStar::TianJi, PalaceName::Caibo) => Some("• Тянь Цзи (Богатство): Заработок умом, анализом или в динамичной сфере торговли."),
                (ZwdsStar::TianJi, PalaceName::Fuqi) => Some("• Тянь Цзи (Супружество): Умный и разговорчивый партнер. Возможны колебания из-за частых перемен."),
                (ZwdsStar::TianJi, PalaceName::Guanlu) => Some("• Тянь Цзи (Карьера): Отлично подходит для IT, исследований, анализа и проектирования."),

                (ZwdsStar::TaiYang, PalaceName::Ming) => Some("• Тай Ян (Жизнь): Страстность, щедрость и стремление к чести. Склонность отдавать все силы другим."),
                (ZwdsStar::TaiYang, PalaceName::Caibo) => Some("• Тай Ян (Богатство): Прибыль через известность, признание или государственную службу."),

                (ZwdsStar::WuQu, PalaceName::Caibo) => Some("• У Цюй (Богатство): Главная финансовая звезда на своем месте. Жесткий прагматизм и надежный доход."),
                (ZwdsStar::WuQu, PalaceName::Ming) => Some("• У Цюй (Жизнь): Решительность, честность и холодный прагматизм. Учитесь гибкости в отношениях."),

                (ZwdsStar::TianTong, PalaceName::Ming) => Some("• Тянь Тон (Жизнь): Мягкий характер, стремление к уюту и гармонии. Опасайтесь лени."),

                (ZwdsStar::LianZhen, PalaceName::Ming) => Some("• Лянь Чжэнь (Жизнь): Сильный стержень, обостренная интуиция и харизма. Упорство в достижении целей."),

                (ZwdsStar::TianFu, PalaceName::Tianzhai) => Some("• Тянь Фу (Имущество): Укрепление семейного очага и успешное вложение в недвижимость."),

                (ZwdsStar::TaiYin, PalaceName::Caibo) => Some("• Тай Инь (Богатство): Постепенное накопление средств. Удачно для инвестиций и сбережения капитала."),

                (ZwdsStar::TanLang, PalaceName::Ming) => Some("• Тань Лан (Жизнь): Разносторонность, обаяние и тяга к эзотерике. Умение заводить связи."),

                (ZwdsStar::JuMen, PalaceName::Ming) => Some("• Джу Мень (Жизнь): Острый ум, аналитика и сила слова. Избегайте споров и недопонимания."),

                (ZwdsStar::QiSha, PalaceName::Ming) => Some("• Ци Ша (Жизнь): Характер полководца. Независимость и смелость, но вероятны периоды одиночества."),

                (ZwdsStar::PoJun, PalaceName::Guanlu) => Some("• По Цзюнь (Карьера): Способность реформировать и начинать с нуля. Подходит для независимого бизнеса."),
                _ => None,
            },
            Locale::En => match (star, palace_name) {
                (ZwdsStar::ZiWei, PalaceName::Ming) => Some("• Zi Wei (Life): Possesses emperor-like qualities with high self-esteem. Dignified and supported by helpers, but should mitigate inner loneliness."),
                (ZwdsStar::ZiWei, PalaceName::Caibo) => Some("• Zi Wei (Wealth): Acquires wealth from high-status environments. Watch out for luxury spending to maintain vanity."),
                (ZwdsStar::ZiWei, PalaceName::Fuqi) => Some("• Zi Wei (Spouse): Partner has strong self-awareness. Guard against power struggles to keep marriage peaceful."),
                (ZwdsStar::ZiWei, PalaceName::Guanlu) => Some("• Zi Wei (Career): Destiny of a leader. Suited for management, independent entrepreneurship, or high-prestige roles."),

                (ZwdsStar::TianJi, PalaceName::Ming) => Some("• Tian Ji (Life): Quick-witted, analytical, and brilliant planner. Knowledge-seeker but prone to anxiety. Keep a peaceful mind."),
                (ZwdsStar::TianJi, PalaceName::Caibo) => Some("• Tian Ji (Wealth): Generates wealth through smart planning, calculations, or rapid circulation. Avoid static financial styles."),
                (ZwdsStar::TianJi, PalaceName::Fuqi) => Some("• Tian Ji (Spouse): Deep affinity with a smart, communicative partner. Mind potential instability due to frequent relocations."),
                (ZwdsStar::TianJi, PalaceName::Guanlu) => Some("• Tian Ji (Career): Highly suited for IT, engineering, research, strategy, and roles demanding intellect."),

                (ZwdsStar::TaiYang, PalaceName::Ming) => Some("• Tai Yang (Life): Warm, public-spirited, and honor-driven. While dedicating yourself to others, take care not to burn out."),
                (ZwdsStar::TaiYang, PalaceName::Caibo) => Some("• Tai Yang (Wealth): Earns wealth through public honor or media representation. Excellent for public-service business."),

                (ZwdsStar::WuQu, PalaceName::Caibo) => Some("• Wu Qu (Wealth): The true wealth star in its place. Controls cash flow and accumulates solid assets through technical execution."),
                (ZwdsStar::WuQu, PalaceName::Ming) => Some("• Wu Qu (Life): Straightforward, decisive, and pragmatic. Cultivate warmth to prevent interpersonal coldness."),

                (ZwdsStar::TianTong, PalaceName::Ming) => Some("• Tian Tong (Life): Gentle, artistic, and peaceful. Blessed with comfort, but needs self-drive to avoid procrastination."),

                (ZwdsStar::LianZhen, PalaceName::Ming) => Some("• Lian Zhen (Life): Highly independent with great artistic sense. Possesses intense ambition and magnetism."),

                (ZwdsStar::TianFu, PalaceName::Tianzhai) => Some("• Tian Fu (Property): The treasury star guards real estate. Highly favorable for purchasing land and accumulating house assets."),

                (ZwdsStar::TaiYin, PalaceName::Caibo) => Some("• Tai Yin (Wealth): Like a steady river, builds fortune gradually through smart savings and real estate passive income."),

                (ZwdsStar::TanLang, PalaceName::Ming) => Some("• Tan Lang (Life): Versatile, magnetic, and social. Attracts relationships and holds deep curiosity about mysterious studies."),

                (ZwdsStar::JuMen, PalaceName::Ming) => Some("• Ju Men (Life): Exceptional analytical power and critical speech. Communicate sincerely to avoid unnecessary disputes."),

                (ZwdsStar::QiSha, PalaceName::Ming) => Some("• Qi Sha (Life): Spirit of a general. Strong breakthrough power and independence. Learn to manage loneliness."),

                (ZwdsStar::PoJun, PalaceName::Guanlu) => Some("• Po Jun (Career): The pioneer star disrupts conventions. Suited for structural reforms or independent startups."),
                _ => None,
            }
        };
        if let Some(desc) = star_desc {
            paragraphs.push(desc.to_string());
        }
    }

    // 4. 사화(SiHua) 결합에 따른 해석
    for star_in_p in stars {
        if let Some(sihua) = star_in_p.si_hua {
            let sihua_desc = match locale {
                Locale::Ko => match sihua {
                    SiHuaType::HuaLu => "✚ 화록(化祿) 결합: 선천적인 복덕과 긍정적인 행운의 통로가 결합되어 이 궁위의 가치가 더욱 유연하고 순조롭게 번창합니다.",
                    SiHuaType::HuaQuan => "✚ 화권(化權) 결합: 강력한 성취 욕구와 투지, 지배력이 주어집니다. 돌파해 내는 추진력이 배가되지만 충돌을 조심해야 합니다.",
                    SiHuaType::HuaKe => "✚ 화과(化科) 결합: 학업적 평판, 귀인의 도움, 혹은 위기관리 능력을 의미하며 안정적이고 조화로운 마무리를 이끕니다.",
                    SiHuaType::HuaJi => "✚ 화기(化忌) 결합: 에너지가 응축 및 정체되어 집착이나 집요한 성격, 지연 장애, 혹은 심적 부채감이 동반되니 차분한 수양이 필요합니다.",
                },
                Locale::Zh => match sihua {
                    SiHuaType::HuaLu => "✚ 化禄交会：带来先天福泽与顺畅财源，使得此宫位的资源更加宽裕与滋润。",
                    SiHuaType::HuaQuan => "✚ 化权交会：赋予强烈的掌控欲、斗志与执行力。冲劲倍增，但需注意摩擦冲突。",
                    SiHuaType::HuaKe => "✚ 化科交会：主名誉声望、贵人暗助，或在危机中获得解脱，引导事务平稳收尾。",
                    SiHuaType::HuaJi => "✚ 化忌交会：能量收缩与阻碍，主执念、延迟、精神负累或波折，需保持平常心以自修。",
                },
                Locale::Ru => match sihua {
                    SiHuaType::HuaLu => "✚ Соединение Хуа Лу: Привносит удачу, изобилие и плавное развитие ресурсов в дела этого дворца.",
                    SiHuaType::HuaQuan => "✚ Соединение Хуа Цюань: Дарует стремление к контролю, силу воли и пробивной характер. Остерегайтесь ссор.",
                    SiHuaType::HuaKe => "✚ Соединение Хуа Кэ: Означает репутацию, славу, академические успехи и помощь покровителей в кризисе.",
                    SiHuaType::HuaJi => "✚ Соединение Хуа Цзи: Сжатие энергии, препятствия, задержки или психологическое напряжение. Требуется терпение.",
                },
                Locale::En => match sihua {
                    SiHuaType::HuaLu => "✚ Hua Lu Effect: Attracts fortune, smooth flow of resources, and flexible prosperity to this palace.",
                    SiHuaType::HuaQuan => "✚ Hua Quan Effect: Grants intense willpower, execution drive, and authority. Boosts breakthroughs but cautions against friction.",
                    SiHuaType::HuaKe => "✚ Hua Ke Effect: Brings reputation, helpful mentors, and crisis resolution, leading to a smooth outcome.",
                    SiHuaType::HuaJi => "✚ Hua Ji Effect: Represents condensed energy, delays, attachments, or mental debts. Demands calm reflection and patience.",
                },
            };
            paragraphs.push(sihua_desc.to_string());
        }
    }

    // 5. 격국(Patterns) 영향성 추가 (명궁에서만 출력)
    if palace_name == PalaceName::Ming && !destiny_patterns.is_empty() {
        let pattern_header = match locale {
            Locale::Ko => "◈ 성립된 격국(格局) 해석:",
            Locale::Zh => "◈ 已成格局解析：",
            Locale::Ru => "◈ Толкование сформированных структур (격국):",
            Locale::En => "◈ Established Destiny Patterns:",
        };
        paragraphs.push(pattern_header.to_string());
        for p in destiny_patterns {
            let p_desc = match locale {
                Locale::Ko => format!("• **{}**: {}", p.name_korean, p.description_korean),
                Locale::Zh => format!("• **{}**: {}", p.name_hanja, p.description_korean),
                Locale::Ru => format!("• **{}**: {}", p.name_korean, p.description_english),
                _ => format!("• **{}**: {}", p.name_korean, p.description_english),
            };
            paragraphs.push(p_desc);
        }
    }

    paragraphs.join("\n\n")
}
