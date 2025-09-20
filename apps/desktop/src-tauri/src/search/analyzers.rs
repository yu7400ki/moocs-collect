use lindera::character_filter::japanese_iteration_mark::JapaneseIterationMarkCharacterFilter;
use lindera::character_filter::unicode_normalize::{
    UnicodeNormalizeCharacterFilter, UnicodeNormalizeKind,
};
use lindera::character_filter::BoxCharacterFilter;
use lindera::dictionary::load_dictionary;
use lindera::mode::Mode;
use lindera::segmenter::Segmenter;
use lindera::token_filter::japanese_base_form::JapaneseBaseFormTokenFilter;
use lindera::token_filter::japanese_katakana_stem::JapaneseKatakanaStemTokenFilter;
use lindera::token_filter::japanese_number::JapaneseNumberTokenFilter;
use lindera::token_filter::japanese_stop_tags::JapaneseStopTagsTokenFilter;
use lindera::token_filter::lowercase::LowercaseTokenFilter;
use lindera::token_filter::BoxTokenFilter;

use lindera_tantivy::tokenizer::LinderaTokenizer;
use tantivy::tokenizer::{LowerCaser, NgramTokenizer, TextAnalyzer};
use tantivy::Index;
use tantivy::TantivyError;

use super::filter::{Kind as NormalizeKind, UnicodeNormalizerTokenFilter};

pub const TOKENIZER_LANG_JA: &str = "lang_ja";
pub const TOKENIZER_BIGRAM: &str = "bigram";

pub fn register_analyzers(index: &Index) -> Result<(), TantivyError> {
    register_japanese_tokenizer(index)?;
    register_bigram_tokenizer(index);
    Ok(())
}

fn register_japanese_tokenizer(index: &Index) -> Result<(), TantivyError> {
    let dictionary = load_dictionary("embedded://unidic").unwrap();
    let user_dictionary = None;
    let mode = Mode::Normal;
    let segmenter = Segmenter::new(mode, dictionary, user_dictionary);

    let unicode_normalize_char_filter =
        UnicodeNormalizeCharacterFilter::new(UnicodeNormalizeKind::NFKC);
    let japanese_iteration_mark_char_filter = JapaneseIterationMarkCharacterFilter::new(true, true);

    let lowercase_token_filter = LowercaseTokenFilter::new();
    let japanese_base_form_token_filter = JapaneseBaseFormTokenFilter::new();
    let japanese_number_token_filter =
        JapaneseNumberTokenFilter::new(Some(vec!["名詞,数詞".to_string()].into_iter().collect()));
    let japanese_stop_tags_token_filter = JapaneseStopTagsTokenFilter::new(
        vec![
            // 接続詞・助詞
            "接続詞",
            "助詞",
            "助詞,格助詞",
            "助詞,係助詞",
            "助詞,副助詞",
            "助詞,間投助詞",
            "助詞,並立助詞",
            "助詞,終助詞",
            "助詞,準体助詞",
            "助詞,接続助詞",
            // 補助記号
            "補助記号",
            "補助記号,一般",
            "補助記号,読点",
            "補助記号,句点",
            "補助記号,空白",
            "補助記号,括弧閉",
            "補助記号,括弧開",
            // フィラー・非言語音
            "感動詞,フィラー",
            "フィラー",
            "非言語音",
            "その他,間投",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
    );
    let japanese_katakana_stem =
        JapaneseKatakanaStemTokenFilter::new(std::num::NonZeroUsize::new(3).unwrap());

    let mut tokenizer = LinderaTokenizer::from_segmenter(segmenter);
    tokenizer
        .append_character_filter(BoxCharacterFilter::from(unicode_normalize_char_filter))
        .append_character_filter(BoxCharacterFilter::from(
            japanese_iteration_mark_char_filter,
        ))
        .append_token_filter(BoxTokenFilter::from(lowercase_token_filter))
        .append_token_filter(BoxTokenFilter::from(japanese_base_form_token_filter))
        .append_token_filter(BoxTokenFilter::from(japanese_number_token_filter))
        .append_token_filter(BoxTokenFilter::from(japanese_stop_tags_token_filter))
        .append_token_filter(BoxTokenFilter::from(japanese_katakana_stem));

    index.tokenizers().register(TOKENIZER_LANG_JA, tokenizer);

    Ok(())
}

fn register_bigram_tokenizer(index: &Index) {
    let normalizer = UnicodeNormalizerTokenFilter::new(NormalizeKind::NFKC);
    let bigram = TextAnalyzer::builder(NgramTokenizer::all_ngrams(2, 2).unwrap())
        .filter(LowerCaser)
        .filter(normalizer)
        .build();

    index.tokenizers().register(TOKENIZER_BIGRAM, bigram);
}
