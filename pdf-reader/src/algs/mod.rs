pub fn find_occurences(text: &str, pat: &str, is_once: bool) -> Vec<u32> {
    const RADIX: u16 = 256;
    fn heuristics(t: &str, size: &usize, badchar: &mut [i32; RADIX as usize]) {
        for i in 0..*size {
            let char_code = t.chars().nth(i).unwrap() as u8;
            badchar[char_code as usize] = i as i32;
        }
    };

    let n = text.len();
    let m = pat.len();

    let mut occurences: Vec<u32> = vec![];

    let mut badchars = [-1; RADIX as usize];

    heuristics(&pat, &m, &mut badchars);
    let mut shift = 0;

    while shift <= (n - m) {
        let mut j = (m - 1) as i32;
        while j >= 0
            && pat.chars().nth(j as usize).unwrap() == text.chars().nth(shift + j as usize).unwrap()
        {
            j -= 1;
        }

        if j < 0 {
            occurences.push(shift as u32);
            if is_once {
                return occurences;
            }
            if shift + m < n {
                let char_code = text.chars().nth(shift + m).unwrap() as u8;
                let res = (m as i32) - (badchars[char_code as usize] as i32);
                shift = (shift as i32 + res).try_into().unwrap_or(0);
            } else {
                shift += 1;
            }
        } else {
            let char_code = text.chars().nth(shift + j as usize).unwrap() as u8;
            shift += std::cmp::max(1, j - badchars[char_code as usize]) as usize;
        }
    }
    occurences
}

#[cfg(test)]
mod algs_tests {
    use super::find_occurences;

    #[test]

    fn find_needle_in_haystack() {
        let text = "ABAAABCD";
        let pat = "ABC";
        let res = find_occurences(text, pat, true);
        assert_eq!(vec![4], res);
    }
    #[test]
    fn find_several_needles_in_haystack() {
        let text = "AABBAABBABCAABBABCAABBABC";
        let pat = "ABC";
        let res = find_occurences(text, pat, false);
        println!("{:?}", res);
        assert_eq!(vec![8, 15, 22], res);
    }

    #[test]

    fn find_needle_in_long_haystack() {
        let text = "ASFASFAEREWWDSGEWEREWFSFEWTEWTGFEWREWEWFWQDVSDVEWREWPOEWPOSDIOJVIOJDSVIOJEWPOIRWOPIQEOPWKOPKXOPZCMPOWMOPRKEWOPKFOP<COPMOPWEMOPEKWROPEWMPOEWNTPOMOPSFKDSOPFKOPEWIWORPEIWOPRJEWNVPEWOJFOPEWGHEWOPREWORIPEWPOKFOPJSFPODSJJIOQPWOPAJPOSJFSNFNSPVSPOEWKOPRJOIFJIOSJFIOSANDIOVIOEWNIFJOEWHTTIOEWMIOMWEPOIEOPWITEWOJNEIONCEWIONcscmdskndoigniongioengwionreiowrniewonfpewfnpsndfpdsnfpdsfpneiweowijrewiurioewjiosdniosvdbvdsuisdhfldskjfioewjreiworjpwekjpowfkskdnlksfndslkfsmdklnlkzdsgdsgdsgkljABC";
        let pat = "ABC";
        let res = find_occurences(text, pat, true);
        assert_eq!(text.len() - pat.len(), res[0] as usize);
    }
}
