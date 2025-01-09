use mini_rusaint::webdynpro::event::{decode_sap_event, SapEventBuilder};

#[test]
fn test_sap_event_decode() {
    let input = "Button_Press~E002Id~E004ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69~003AVIW_MAIN.BUTTON_PREV~E003~E002ResponseData~E004delta~E005ClientAction~E004submit~E003~E002~E003";
    let expected = "Button_Press{Id:ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.BUTTON_PREV}{ResponseData:delta,ClientAction:submit}{}";
    assert_eq!(decode_sap_event(input), expected);
}

#[test]
fn test_sap_event_builder() {
    let sap_event = SapEventBuilder::default()
        .event("ComboBox")
        .control("Select")
        .add_parameter((
            "Id".to_string(),
            "ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.PERYR".to_string(),
        ))
        .add_parameter(("Key".to_string(), "2024".to_string()))
        .add_ucf_parameter(("ResponseData".to_string(), "delta".to_string()))
        .add_ucf_parameter(("ClientAction".to_string(), "submit".to_string()))
        .build()
        .unwrap();

    assert_eq!(sap_event.event, "ComboBox");
    assert_eq!(sap_event.control, "Select");
    assert_eq!(
        sap_event.parameters.get("Id").unwrap(),
        "ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69:VIW_MAIN.PERYR"
    );
    assert_eq!(
        sap_event.ucf_parameters.get("ResponseData").unwrap(),
        "delta"
    );

    // Test the Display implementation
    let sap_event_str = sap_event.to_string();

    assert!(sap_event_str.contains("ComboBox_Select"));
    assert!(sap_event_str
        .contains("Id~E004ZCMW_PERIOD_RE.ID_0DC742680F42DA9747594D1AE51A0C69~003AVIW_MAIN.PERYR"));
    assert!(sap_event_str.contains("Key~E0042024"));
    assert!(sap_event_str.contains("ResponseData~E004delta"));
    assert!(sap_event_str.contains("ClientAction~E004submit"));
    assert!(sap_event_str.contains("~E002~E003"));
}
