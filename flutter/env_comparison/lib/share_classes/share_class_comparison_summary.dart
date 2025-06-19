typedef ShareClassComparisonSummary = ({
  String dataType,
  List<ShareClassComparison> base,
  List<ShareClassComparison> comparison,
  List<ShareClassComparisonResult> result,
});

typedef ShareClassComparison = ({String isinCode, String mappedJson});

typedef ShareClassComparisonResult = ({
  String action,
  String isinCode,
  String mappedJson,
});
