typedef MediaOutletMapComparisonSummary = ({
  String dataType,
  List<MediaOutletMapComparison> base,
  List<MediaOutletMapComparison> comparison,
  List<MediaOutletMapComparisonResult> result,
});

typedef MediaOutletMapComparison = ({
  String mediaOutletName,
  String isinCode,
  String currency,
  String status,
  DateTime? cancellationDate,
  String? cancellationReason,
});

typedef MediaOutletMapComparisonResult = ({
  String action,
  String mediaOutletName,
  String isinCode,
  String currency,
  String status,
  DateTime? cancellationDate,
  String? cancellationReason,
});
