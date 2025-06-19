typedef TopicComparisonSummary = ({
  String dataType,
  List<TopicComparison> base,
  List<TopicComparison> comparison,
  List<TopicComparisonResult> result,
});

typedef TopicComparison = ({
  String isinCode,
  String currency,
  String clientId,
  String mappedJson,
  String status,
  String lastChangeType,
  String lastChangeProcessed,
});

typedef TopicComparisonResult = ({
  String action,
  String isinCode,
  String currency,
  String mappedJson,
  String lastChangeType,
  String lastChangeProcessed,
});
