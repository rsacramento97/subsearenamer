#[cfg(test)]
mod tests {
    use super::super::bridge_contract::*;

    #[test]
    fn request_serializes_with_frontend_field_names() {
        let request = RenameJobRequest {
            source_dir: "C:/source".into(),
            destination_dir: "D:/dest".into(),
            timezone: "UTC".into(),
            manual_offset_minutes: Some(-180),
            verify_hash: true,
        };
        let json = serde_json::to_value(request).unwrap();
        assert_eq!(json["sourceDir"], "C:/source");
        assert_eq!(json["destinationDir"], "D:/dest");
        assert_eq!(json["manualOffsetMinutes"], -180);
        assert_eq!(json["verifyHash"], true);
    }

    #[test]
    fn preview_status_is_stable() {
        assert_eq!(serde_json::to_string(&PreviewStatus::Ready).unwrap(), "\"ready\"");
        assert_eq!(serde_json::to_string(&PreviewStatus::Conflict).unwrap(), "\"conflict\"");
        assert_eq!(serde_json::to_string(&PreviewStatus::Invalid).unwrap(), "\"invalid\"");
    }
}
