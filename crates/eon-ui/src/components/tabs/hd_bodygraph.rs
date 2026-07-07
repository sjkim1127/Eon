use dioxus::prelude::*;
use eon_human_design::{HdCenter, HumanDesignResult};

#[component]
pub fn HdBodyGraph(result: HumanDesignResult) -> Element {
    let has_center = move |c: HdCenter| result.defined_centers.contains(&c);

    // Determine if a gate is activated in personality (black), design (red), or both (striped/purple)
    let get_gate_color = move |g: u8| -> &'static str {
        let is_pers = result.personality.values().any(|p| p.gate == g);
        let is_des = result.design.values().any(|p| p.gate == g);
        if is_pers && is_des {
            "#a855f7"
        }
        // purple-500 (both)
        else if is_pers {
            "#1e293b"
        }
        // slate-800 (personality/black)
        else if is_des {
            "#f43f5e"
        }
        // rose-500 (design/red)
        else {
            "#e2e8f0"
        } // slate-200 (empty)
    };

    let defined_fill = "#0d9488";
    let open_fill = "#f8fafc";
    let outline_color = "#334155";

    rsx! {
        div { class: "w-full flex justify-center items-center py-6 bg-slate-900/30 rounded-2xl border border-slate-800",
            svg {
                width: "100%", height: "auto", view_box: "0 0 400 600", class: "drop-shadow-2xl max-w-sm",
                // Channel 1-8
                line { x1: "200", y1: "310", x2: "200.0", y2: "290.0", stroke: get_gate_color(1), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "200.0", y1: "290.0", x2: "200", y2: "270", stroke: get_gate_color(8), stroke_width: "6", stroke_linecap: "round" }
                text { x: "196.0", y: "308.0", class: "text-[8px] font-bold fill-slate-500", "1" }
                text { x: "196.0", y: "276.0", class: "text-[8px] font-bold fill-slate-500", "8" }
                // Channel 2-14
                line { x1: "220", y1: "370", x2: "215.0", y2: "390.0", stroke: get_gate_color(2), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "215.0", y1: "390.0", x2: "210", y2: "410", stroke: get_gate_color(14), stroke_width: "6", stroke_linecap: "round" }
                text { x: "215.0", y: "376.0", class: "text-[8px] font-bold fill-slate-500", "2" }
                text { x: "207.0", y: "408.0", class: "text-[8px] font-bold fill-slate-500", "14" }
                // Channel 3-60
                line { x1: "200", y1: "460", x2: "200.0", y2: "485.0", stroke: get_gate_color(3), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "200.0", y1: "485.0", x2: "200", y2: "510", stroke: get_gate_color(60), stroke_width: "6", stroke_linecap: "round" }
                text { x: "196.0", y: "467.0", class: "text-[8px] font-bold fill-slate-500", "3" }
                text { x: "196.0", y: "507.0", class: "text-[8px] font-bold fill-slate-500", "60" }
                // Channel 4-63
                line { x1: "220", y1: "120", x2: "220.0", y2: "100.0", stroke: get_gate_color(4), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "220.0", y1: "100.0", x2: "220", y2: "80", stroke: get_gate_color(63), stroke_width: "6", stroke_linecap: "round" }
                text { x: "216.0", y: "118.0", class: "text-[8px] font-bold fill-slate-500", "4" }
                text { x: "216.0", y: "86.0", class: "text-[8px] font-bold fill-slate-500", "63" }
                // Channel 5-15
                line { x1: "200", y1: "410", x2: "200.0", y2: "395.0", stroke: get_gate_color(5), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "200.0", y1: "395.0", x2: "200", y2: "380", stroke: get_gate_color(15), stroke_width: "6", stroke_linecap: "round" }
                text { x: "196.0", y: "409.0", class: "text-[8px] font-bold fill-slate-500", "5" }
                text { x: "196.0", y: "385.0", class: "text-[8px] font-bold fill-slate-500", "15" }
                // Channel 6-59
                line { x1: "290", y1: "440", x2: "257.5", y2: "440.0", stroke: get_gate_color(6), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "257.5", y1: "440.0", x2: "225", y2: "440", stroke: get_gate_color(59), stroke_width: "6", stroke_linecap: "round" }
                text { x: "279.5", y: "442.0", class: "text-[8px] font-bold fill-slate-500", "6" }
                text { x: "227.5", y: "442.0", class: "text-[8px] font-bold fill-slate-500", "59" }
                // Channel 7-31
                line { x1: "185", y1: "325", x2: "182.5", y2: "297.5", stroke: get_gate_color(7), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "182.5", y1: "297.5", x2: "180", y2: "270", stroke: get_gate_color(31), stroke_width: "6", stroke_linecap: "round" }
                text { x: "180.5", y: "321.5", class: "text-[8px] font-bold fill-slate-500", "7" }
                text { x: "176.5", y: "277.5", class: "text-[8px] font-bold fill-slate-500", "31" }
                // Channel 9-52
                line { x1: "210", y1: "460", x2: "212.5", y2: "485.0", stroke: get_gate_color(9), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "212.5", y1: "485.0", x2: "215", y2: "510", stroke: get_gate_color(52), stroke_width: "6", stroke_linecap: "round" }
                text { x: "206.5", y: "467.0", class: "text-[8px] font-bold fill-slate-500", "9" }
                text { x: "210.5", y: "507.0", class: "text-[8px] font-bold fill-slate-500", "52" }
                // Channel 10-20
                line { x1: "180", y1: "360", x2: "185.0", y2: "315.0", stroke: get_gate_color(10), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "185.0", y1: "315.0", x2: "190", y2: "270", stroke: get_gate_color(20), stroke_width: "6", stroke_linecap: "round" }
                text { x: "177.0", y: "353.0", class: "text-[8px] font-bold fill-slate-500", "10" }
                text { x: "185.0", y: "281.0", class: "text-[8px] font-bold fill-slate-500", "20" }
                // Channel 10-34
                line { x1: "180", y1: "360", x2: "185.0", y2: "385.0", stroke: get_gate_color(10), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "185.0", y1: "385.0", x2: "190", y2: "410", stroke: get_gate_color(34), stroke_width: "6", stroke_linecap: "round" }
                text { x: "177.0", y: "367.0", class: "text-[8px] font-bold fill-slate-500", "10" }
                text { x: "185.0", y: "407.0", class: "text-[8px] font-bold fill-slate-500", "34" }
                // Channel 10-57
                line { x1: "180", y1: "360", x2: "150.0", y2: "385.0", stroke: get_gate_color(10), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "150.0", y1: "385.0", x2: "120", y2: "410", stroke: get_gate_color(57), stroke_width: "6", stroke_linecap: "round" }
                text { x: "170.0", y: "367.0", class: "text-[8px] font-bold fill-slate-500", "10" }
                text { x: "122.0", y: "407.0", class: "text-[8px] font-bold fill-slate-500", "57" }
                // Channel 11-56
                line { x1: "230", y1: "160", x2: "222.5", y2: "190.0", stroke: get_gate_color(11), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "222.5", y1: "190.0", x2: "215", y2: "220", stroke: get_gate_color(56), stroke_width: "6", stroke_linecap: "round" }
                text { x: "224.5", y: "168.0", class: "text-[8px] font-bold fill-slate-500", "11" }
                text { x: "212.5", y: "216.0", class: "text-[8px] font-bold fill-slate-500", "56" }
                // Channel 12-22
                line { x1: "225", y1: "245", x2: "247.5", y2: "327.5", stroke: get_gate_color(12), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "247.5", y1: "327.5", x2: "270", y2: "410", stroke: get_gate_color(22), stroke_width: "6", stroke_linecap: "round" }
                text { x: "225.5", y: "263.5", class: "text-[8px] font-bold fill-slate-500", "12" }
                text { x: "261.5", y: "395.5", class: "text-[8px] font-bold fill-slate-500", "22" }
                // Channel 13-33
                line { x1: "215", y1: "325", x2: "212.5", y2: "297.5", stroke: get_gate_color(13), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "212.5", y1: "297.5", x2: "210", y2: "270", stroke: get_gate_color(33), stroke_width: "6", stroke_linecap: "round" }
                text { x: "210.5", y: "321.5", class: "text-[8px] font-bold fill-slate-500", "13" }
                text { x: "206.5", y: "277.5", class: "text-[8px] font-bold fill-slate-500", "33" }
                // Channel 16-48
                line { x1: "175", y1: "245", x2: "142.5", y2: "327.5", stroke: get_gate_color(16), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "142.5", y1: "327.5", x2: "110", y2: "410", stroke: get_gate_color(48), stroke_width: "6", stroke_linecap: "round" }
                text { x: "164.5", y: "263.5", class: "text-[8px] font-bold fill-slate-500", "16" }
                text { x: "112.5", y: "395.5", class: "text-[8px] font-bold fill-slate-500", "48" }
                // Channel 17-62
                line { x1: "230", y1: "140", x2: "207.5", y2: "180.0", stroke: get_gate_color(17), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "207.5", y1: "180.0", x2: "185", y2: "220", stroke: get_gate_color(62), stroke_width: "6", stroke_linecap: "round" }
                text { x: "221.5", y: "150.0", class: "text-[8px] font-bold fill-slate-500", "17" }
                text { x: "185.5", y: "214.0", class: "text-[8px] font-bold fill-slate-500", "62" }
                // Channel 18-58
                line { x1: "80", y1: "470", x2: "127.5", y2: "510.0", stroke: get_gate_color(18), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "127.5", y1: "510.0", x2: "175", y2: "550", stroke: get_gate_color(58), stroke_width: "6", stroke_linecap: "round" }
                text { x: "85.5", y: "480.0", class: "text-[8px] font-bold fill-slate-500", "18" }
                text { x: "161.5", y: "544.0", class: "text-[8px] font-bold fill-slate-500", "58" }
                // Channel 19-49
                line { x1: "225", y1: "520", x2: "262.5", y2: "487.5", stroke: get_gate_color(19), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "262.5", y1: "487.5", x2: "300", y2: "455", stroke: get_gate_color(49), stroke_width: "6", stroke_linecap: "round" }
                text { x: "228.5", y: "515.5", class: "text-[8px] font-bold fill-slate-500", "19" }
                text { x: "288.5", y: "463.5", class: "text-[8px] font-bold fill-slate-500", "49" }
                // Channel 20-34
                line { x1: "190", y1: "270", x2: "190.0", y2: "340.0", stroke: get_gate_color(20), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "190.0", y1: "340.0", x2: "190", y2: "410", stroke: get_gate_color(34), stroke_width: "6", stroke_linecap: "round" }
                text { x: "186.0", y: "286.0", class: "text-[8px] font-bold fill-slate-500", "20" }
                text { x: "186.0", y: "398.0", class: "text-[8px] font-bold fill-slate-500", "34" }
                // Channel 20-57
                line { x1: "190", y1: "270", x2: "155.0", y2: "340.0", stroke: get_gate_color(20), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "155.0", y1: "340.0", x2: "120", y2: "410", stroke: get_gate_color(57), stroke_width: "6", stroke_linecap: "round" }
                text { x: "179.0", y: "286.0", class: "text-[8px] font-bold fill-slate-500", "20" }
                text { x: "123.0", y: "398.0", class: "text-[8px] font-bold fill-slate-500", "57" }
                // Channel 21-45
                line { x1: "265", y1: "300", x2: "245.0", y2: "265.0", stroke: get_gate_color(21), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "245.0", y1: "265.0", x2: "225", y2: "230", stroke: get_gate_color(45), stroke_width: "6", stroke_linecap: "round" }
                text { x: "257.0", y: "295.0", class: "text-[8px] font-bold fill-slate-500", "21" }
                text { x: "225.0", y: "239.0", class: "text-[8px] font-bold fill-slate-500", "45" }
                // Channel 23-43
                line { x1: "200", y1: "220", x2: "185.0", y2: "180.0", stroke: get_gate_color(23), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "185.0", y1: "180.0", x2: "170", y2: "140", stroke: get_gate_color(43), stroke_width: "6", stroke_linecap: "round" }
                text { x: "193.0", y: "214.0", class: "text-[8px] font-bold fill-slate-500", "23" }
                text { x: "169.0", y: "150.0", class: "text-[8px] font-bold fill-slate-500", "43" }
                // Channel 24-61
                line { x1: "200", y1: "120", x2: "200.0", y2: "100.0", stroke: get_gate_color(24), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "200.0", y1: "100.0", x2: "200", y2: "80", stroke: get_gate_color(61), stroke_width: "6", stroke_linecap: "round" }
                text { x: "196.0", y: "118.0", class: "text-[8px] font-bold fill-slate-500", "24" }
                text { x: "196.0", y: "86.0", class: "text-[8px] font-bold fill-slate-500", "61" }
                // Channel 25-51
                line { x1: "225", y1: "325", x2: "240.0", y2: "322.5", stroke: get_gate_color(25), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "240.0", y1: "322.5", x2: "255", y2: "320", stroke: get_gate_color(51), stroke_width: "6", stroke_linecap: "round" }
                text { x: "224.0", y: "326.5", class: "text-[8px] font-bold fill-slate-500", "25" }
                text { x: "248.0", y: "322.5", class: "text-[8px] font-bold fill-slate-500", "51" }
                // Channel 26-44
                line { x1: "260", y1: "335", x2: "195.0", y2: "372.5", stroke: get_gate_color(26), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "195.0", y1: "372.5", x2: "130", y2: "410", stroke: get_gate_color(44), stroke_width: "6", stroke_linecap: "round" }
                text { x: "243.0", y: "344.5", class: "text-[8px] font-bold fill-slate-500", "26" }
                text { x: "139.0", y: "404.5", class: "text-[8px] font-bold fill-slate-500", "44" }
                // Channel 27-50
                line { x1: "175", y1: "440", x2: "145.0", y2: "432.5", stroke: get_gate_color(27), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "145.0", y1: "432.5", x2: "115", y2: "425", stroke: get_gate_color(50), stroke_width: "6", stroke_linecap: "round" }
                text { x: "165.0", y: "440.5", class: "text-[8px] font-bold fill-slate-500", "27" }
                text { x: "117.0", y: "428.5", class: "text-[8px] font-bold fill-slate-500", "50" }
                // Channel 28-38
                line { x1: "95", y1: "455", x2: "135.0", y2: "495.0", stroke: get_gate_color(28), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "135.0", y1: "495.0", x2: "175", y2: "535", stroke: get_gate_color(38), stroke_width: "6", stroke_linecap: "round" }
                text { x: "99.0", y: "465.0", class: "text-[8px] font-bold fill-slate-500", "28" }
                text { x: "163.0", y: "529.0", class: "text-[8px] font-bold fill-slate-500", "38" }
                // Channel 29-46
                line { x1: "225", y1: "420", x2: "220.0", y2: "390.0", stroke: get_gate_color(29), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "220.0", y1: "390.0", x2: "215", y2: "360", stroke: get_gate_color(46), stroke_width: "6", stroke_linecap: "round" }
                text { x: "220.0", y: "416.0", class: "text-[8px] font-bold fill-slate-500", "29" }
                text { x: "212.0", y: "368.0", class: "text-[8px] font-bold fill-slate-500", "46" }
                // Channel 30-41
                line { x1: "320", y1: "470", x2: "272.5", y2: "510.0", stroke: get_gate_color(30), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "272.5", y1: "510.0", x2: "225", y2: "550", stroke: get_gate_color(41), stroke_width: "6", stroke_linecap: "round" }
                text { x: "306.5", y: "480.0", class: "text-[8px] font-bold fill-slate-500", "30" }
                text { x: "230.5", y: "544.0", class: "text-[8px] font-bold fill-slate-500", "41" }
                // Channel 32-54
                line { x1: "105", y1: "440", x2: "140.0", y2: "480.0", stroke: get_gate_color(32), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "140.0", y1: "480.0", x2: "175", y2: "520", stroke: get_gate_color(54), stroke_width: "6", stroke_linecap: "round" }
                text { x: "108.0", y: "450.0", class: "text-[8px] font-bold fill-slate-500", "32" }
                text { x: "164.0", y: "514.0", class: "text-[8px] font-bold fill-slate-500", "54" }
                // Channel 34-57
                line { x1: "190", y1: "410", x2: "155.0", y2: "410.0", stroke: get_gate_color(34), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "155.0", y1: "410.0", x2: "120", y2: "410", stroke: get_gate_color(57), stroke_width: "6", stroke_linecap: "round" }
                text { x: "179.0", y: "412.0", class: "text-[8px] font-bold fill-slate-500", "34" }
                text { x: "123.0", y: "412.0", class: "text-[8px] font-bold fill-slate-500", "57" }
                // Channel 35-36
                line { x1: "225", y1: "260", x2: "252.5", y2: "335.0", stroke: get_gate_color(35), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "252.5", y1: "335.0", x2: "280", y2: "410", stroke: get_gate_color(36), stroke_width: "6", stroke_linecap: "round" }
                text { x: "226.5", y: "277.0", class: "text-[8px] font-bold fill-slate-500", "35" }
                text { x: "270.5", y: "397.0", class: "text-[8px] font-bold fill-slate-500", "36" }
                // Channel 37-40
                line { x1: "285", y1: "425", x2: "282.5", y2: "372.5", stroke: get_gate_color(37), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "282.5", y1: "372.5", x2: "280", y2: "320", stroke: get_gate_color(40), stroke_width: "6", stroke_linecap: "round" }
                text { x: "280.5", y: "416.5", class: "text-[8px] font-bold fill-slate-500", "37" }
                text { x: "276.5", y: "332.5", class: "text-[8px] font-bold fill-slate-500", "40" }
                // Channel 39-55
                line { x1: "225", y1: "535", x2: "267.5", y2: "502.5", stroke: get_gate_color(39), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "267.5", y1: "502.5", x2: "310", y2: "470", stroke: get_gate_color(55), stroke_width: "6", stroke_linecap: "round" }
                text { x: "229.5", y: "530.5", class: "text-[8px] font-bold fill-slate-500", "39" }
                text { x: "297.5", y: "478.5", class: "text-[8px] font-bold fill-slate-500", "55" }
                // Channel 42-53
                line { x1: "190", y1: "460", x2: "187.5", y2: "485.0", stroke: get_gate_color(42), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "187.5", y1: "485.0", x2: "185", y2: "510", stroke: get_gate_color(53), stroke_width: "6", stroke_linecap: "round" }
                text { x: "185.5", y: "467.0", class: "text-[8px] font-bold fill-slate-500", "42" }
                text { x: "181.5", y: "507.0", class: "text-[8px] font-bold fill-slate-500", "53" }
                // Channel 47-64
                line { x1: "180", y1: "120", x2: "180.0", y2: "100.0", stroke: get_gate_color(47), stroke_width: "6", stroke_linecap: "round" }
                line { x1: "180.0", y1: "100.0", x2: "180", y2: "80", stroke: get_gate_color(64), stroke_width: "6", stroke_linecap: "round" }
                text { x: "176.0", y: "118.0", class: "text-[8px] font-bold fill-slate-500", "47" }
                text { x: "176.0", y: "86.0", class: "text-[8px] font-bold fill-slate-500", "64" }
                // Head
                polygon { points: "200,20 240,80 160,80", fill: if has_center(HdCenter::Head) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
                // Ajna
                polygon { points: "160,120 240,120 200,180", fill: if has_center(HdCenter::Ajna) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
                // Throat
                rect { x: "175", y: "220", width: "50", height: "50", rx: "8", fill: if has_center(HdCenter::Throat) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
                // SelfG
                polygon { points: "200,310 235,345 200,380 165,345", fill: if has_center(HdCenter::SelfG) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
                // Heart
                polygon { points: "255,300 285,300 270,340", fill: if has_center(HdCenter::Heart) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
                // Sacral
                rect { x: "175", y: "410", width: "50", height: "50", rx: "8", fill: if has_center(HdCenter::Sacral) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
                // Root
                rect { x: "175", y: "510", width: "50", height: "50", rx: "8", fill: if has_center(HdCenter::Root) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
                // Spleen
                polygon { points: "80,410 130,410 80,470", fill: if has_center(HdCenter::Spleen) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
                // SolarPlexus
                polygon { points: "320,410 270,410 320,470", fill: if has_center(HdCenter::SolarPlexus) { defined_fill } else { open_fill }, stroke: outline_color, stroke_width: "4" }
            }
        }
    }
}
