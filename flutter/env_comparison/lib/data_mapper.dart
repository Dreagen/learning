import 'dart:core';

import 'package:env_comparison/main.dart';
import 'package:env_comparison/repository.dart';

class DataMapper {
  List<ComparisonSummaryForTable> mapFromComparisonSummary(
    ComparisonSummary summary,
  ) {
    var dataForTable = <ComparisonSummaryForTable>{};

    dataForTable.add((
      dataType: summary.topicComparisonSummary.dataType,
      baseChanges: summary.topicComparisonSummary.base.length,
      comparisonChanges: summary.topicComparisonSummary.comparison.length,
      resultAdds: summary.topicComparisonSummary.result
          .where((x) => x.action == "ADD")
          .length,
      resultMods: summary.topicComparisonSummary.result
          .where((x) => x.action == "MOD")
          .length,
      resultDels: summary.topicComparisonSummary.result
          .where((x) => x.action == "DEL")
          .length,
    ));

    dataForTable.add((
      dataType: summary.shareClassComparisonSummary.dataType,
      baseChanges: summary.shareClassComparisonSummary.base.length,
      comparisonChanges: summary.shareClassComparisonSummary.comparison.length,
      resultAdds: summary.shareClassComparisonSummary.result
          .where((x) => x.action == "ADD")
          .length,
      resultMods: summary.shareClassComparisonSummary.result
          .where((x) => x.action == "MOD")
          .length,
      resultDels: summary.shareClassComparisonSummary.result
          .where((x) => x.action == "DEL")
          .length,
    ));

    return dataForTable.toList();
  }
}
