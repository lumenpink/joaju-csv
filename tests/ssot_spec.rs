use ssot_rs::{Dataset, SsotError};

#[test]
fn test_parse_csv_basic() {
    let csv = "id,pai,mae,sex,geracao,fenotipo\n1,0,0,M,1,wt\n2,0,0,F,1,wt\n3,1,2,F,2,afetado\n";
    let ds = Dataset::from_csv(csv).expect("parsing falhou");

    assert_eq!(
        ds.headers(),
        &["id", "pai", "mae", "sex", "geracao", "fenotipo"]
    );
    assert_eq!(ds.row_count(), 3);
    assert_eq!(ds.column_count(), 6);
}

#[test]
fn test_select_projection_and_alias() {
    let csv = "id,pai,mae,sex,geracao,fenotipo_abo,label_abo\n1,0,0,M,1,B,I-1\n2,0,0,F,1,A,I-2\n3,1,2,F,2,AB,II-1\n";
    let ds = Dataset::from_csv(csv).expect("parsing falhou");

    let projected = ds
        .apply_select("id, pai, mae, sex, fenotipo_abo as fenotipo, label_abo as label")
        .expect("select falhou");

    assert_eq!(
        projected.headers(),
        &["id", "pai", "mae", "sex", "fenotipo", "label"]
    );
    assert_eq!(projected.row_count(), 3);
    assert_eq!(
        projected.get_row(0).unwrap(),
        &["1", "0", "0", "M", "B", "I-1"]
    );
    assert_eq!(
        projected.get_row(2).unwrap(),
        &["3", "1", "2", "F", "AB", "II-1"]
    );
}

#[test]
fn test_where_numeric_filter() {
    let csv = "id,geracao,status\n1,1,ativo\n2,2,ativo\n3,3,ativo\n4,4,inativo\n5,5,inativo\n";
    let ds = Dataset::from_csv(csv).expect("parsing falhou");

    let filtered = ds.apply_where("geracao <= 3").expect("where falhou");
    assert_eq!(filtered.row_count(), 3);
    assert_eq!(filtered.get_row(0).unwrap(), &["1", "1", "ativo"]);
    assert_eq!(filtered.get_row(2).unwrap(), &["3", "3", "ativo"]);
}

#[test]
fn test_where_text_filter_and_multiple_clauses() {
    let csv = "id,geracao,sex,status\n1,1,M,ativo\n2,1,F,ativo\n3,2,M,inativo\n4,2,F,ativo\n5,3,F,inativo\n";
    let ds = Dataset::from_csv(csv).expect("parsing falhou");

    let filtered = ds
        .apply_where("sex == F and status == ativo")
        .expect("where falhou");

    assert_eq!(filtered.row_count(), 2);
    assert_eq!(filtered.get_row(0).unwrap(), &["2", "1", "F", "ativo"]);
    assert_eq!(filtered.get_row(1).unwrap(), &["4", "2", "F", "ativo"]);
}

#[test]
fn test_where_and_select_combined() {
    let csv = "id,pai,mae,sex,geracao,fenotipo_olhos\n1,0,0,M,1,castanho\n2,0,0,F,1,azul\n3,1,2,F,2,azul\n4,1,2,M,2,castanho\n5,0,0,F,3,verde\n";
    let ds = Dataset::from_csv(csv).expect("parsing falhou");

    let result = ds
        .apply_query(
            Some("id, pai, mae, sex, fenotipo_olhos as fenotipo"),
            Some("geracao <= 2 and fenotipo_olhos == azul"),
        )
        .expect("query falhou");

    assert_eq!(result.headers(), &["id", "pai", "mae", "sex", "fenotipo"]);
    assert_eq!(result.row_count(), 2);
    assert_eq!(result.get_row(0).unwrap(), &["2", "0", "0", "F", "azul"]);
    assert_eq!(result.get_row(1).unwrap(), &["3", "1", "2", "F", "azul"]);
}

#[test]
fn test_parse_block_directives() {
    let block = r#"%% src=dados/familia.csv
%% select=id, pai, mae, sex, fenotipo_abo as fenotipo
%% where=geracao <= 3
%% layout.width=95%
"#;
    let directives = ssot_rs::parse_block_directives(block);
    assert_eq!(directives.get("src").unwrap(), "dados/familia.csv");
    assert_eq!(
        directives.get("select").unwrap(),
        "id, pai, mae, sex, fenotipo_abo as fenotipo"
    );
    assert_eq!(directives.get("where").unwrap(), "geracao <= 3");
    assert_eq!(directives.get("layout.width").unwrap(), "95%");
}

#[test]
fn test_to_csv_output() {
    let csv = "id,nome\n1,Lumen\n2,Pink\n";
    let ds = Dataset::from_csv(csv).unwrap();
    let exported = ds.to_csv().unwrap();
    assert!(exported.contains("id,nome"));
    assert!(exported.contains("1,Lumen"));
    assert!(exported.contains("2,Pink"));
}

#[test]
fn test_error_on_unknown_column() {
    let csv = "id,nome\n1,Lumen\n";
    let ds = Dataset::from_csv(csv).unwrap();
    let err = ds.apply_select("id, coluna_inexistente").unwrap_err();
    match err {
        SsotError::ColumnNotFound(col) => assert_eq!(col, "coluna_inexistente"),
        other => panic!("Erro inesperado: {:?}", other),
    }
}

#[test]
fn test_pedigree_official_and_pt_synonyms_in_select() {
    // CSV original com nomes oficiais em inglês (pedtools/LINKAGE)
    let csv = "id,fid,mid,sex,aff,proband\n1,0,0,1,1,0\n2,0,0,2,1,0\n3,1,2,2,2,1\n";
    let ds = Dataset::from_csv(csv).expect("parsing falhou");

    // Consulta usando adaptações em português
    let projected = ds
        .apply_select("id, pai, mae, sexo, aff as fenotipo, probando")
        .expect("select com sinônimos falhou");

    assert_eq!(
        projected.headers(),
        &["id", "pai", "mae", "sexo", "fenotipo", "probando"]
    );
    assert_eq!(
        projected.get_row(2).unwrap(),
        &["3", "1", "2", "2", "2", "1"]
    );
}

#[test]
fn test_canonicalize_pedigree_headers() {
    // Oficial -> Português
    let csv_official = "id,fid,mid,sex,aff,proband\n1,0,0,1,1,0\n";
    let ds_off = Dataset::from_csv(csv_official).unwrap();
    let ds_pt = ds_off.canonicalize_pedigree_pt();
    assert_eq!(
        ds_pt.headers(),
        &["id", "pai", "mae", "sexo", "fenotipo", "probando"]
    );

    // Português -> Oficial
    let csv_pt = "id,pai,mae,sexo,fenotipo,probando\n1,0,0,1,1,0\n";
    let ds_pt_in = Dataset::from_csv(csv_pt).unwrap();
    let ds_off_out = ds_pt_in.canonicalize_pedigree_official();
    assert_eq!(
        ds_off_out.headers(),
        &["id", "fid", "mid", "sex", "aff", "proband"]
    );
}
