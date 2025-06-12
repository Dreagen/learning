class Repository {
  Future<List<ComparisonSummary>> getItems() async {
    var topicData = createFakeTopicComparison();
    var shareClassData = createFakeShareClassData();

    return [
      (
        topicComparisonSummary: topicData,
        shareClassComparisonSummary: shareClassData,
      ),
      (
        topicComparisonSummary: topicData,
        shareClassComparisonSummary: shareClassData,
      ),
    ];
  }

  TopicComparisonSummary createFakeTopicComparison() {
    final base1 = (
      isinCode: 'LU0808551500',
      currency: 'EUR',
      clientId: 'FECC23FA-5D41-4156-A170-91D7A6F3C7BB',
      mappedJson: '{"pubcry":"EUR"}',
      status: 'Active',
      lastChangeType: 'ADD',
      lastChangeProcessed: 'true',
    );

    final base2 = (
      isinCode: 'LU1227550453',
      currency: 'EUR',
      clientId: 'FECC23FA-5D41-4156-A170-91D7A6F3C7BB',
      mappedJson: '{"pubcry":"EUR"}',
      status: 'Active',
      lastChangeType: 'ADD',
      lastChangeProcessed: 'true',
    );

    final base3 = (
      isinCode: 'LU2407564710',
      currency: 'USD',
      clientId: 'FECC23FA-5D41-4156-A170-91D7A6F3C7BB',
      mappedJson: '{"pubcry":"USD"}',
      status: 'Active',
      lastChangeType: 'ADD',
      lastChangeProcessed: 'true',
    );

    final comparison1 = (
      isinCode: 'LU0808551500',
      currency: 'EUR',
      clientId: 'FECC23FA-5D41-4156-A170-91D7A6F3C7BB',
      mappedJson: '{"pubcry":"EUR"}',
      status: 'Active',
      lastChangeType: 'ADD',
      lastChangeProcessed: 'false',
    );

    final comparison2 = (
      isinCode: 'LU1227550453',
      currency: 'EUR',
      clientId: 'FECC23FA-5D41-4156-A170-91D7A6F3C7BB',
      mappedJson: '{"pubcry":"EUR"}',
      status: 'Active',
      lastChangeType: 'ADD',
      lastChangeProcessed: 'false',
    );

    final comparison3 = (
      isinCode: 'LU2407564710',
      currency: 'USD',
      clientId: 'FECC23FA-5D41-4156-A170-91D7A6F3C7BB',
      mappedJson: '{"pubcry":"USD"}',
      status: 'Active',
      lastChangeType: 'ADD',
      lastChangeProcessed: 'false',
    );

    final comparisonResult1 = (
      action: 'MOD',
      isinCode: 'LU1227550453',
      currency: 'EUR',
      mappedJson: '{"pubcry":"EUR"}',
      lastChangeType: 'ADD',
      lastChangeProcessed: 'DIFF(true->false)',
    );

    final comparisonResult2 = (
      action: 'MOD',
      isinCode: 'LU2407564710',
      currency: 'USD',
      mappedJson: '{"pubcry":"USD"}',
      lastChangeType: 'ADD',
      lastChangeProcessed: 'DIFF(true->false)',
    );

    final comparisonResult3 = (
      action: 'MOD',
      isinCode: 'LU0808551500',
      currency: 'EUR',
      mappedJson: '{"pubcry":"EUR"}',
      lastChangeType: 'ADD',
      lastChangeProcessed: 'DIFF(true->false)',
    );

    return (
      dataType: "topic",
      base: [base1, base2, base3],
      comparison: [comparison1, comparison2, comparison3],
      result: [comparisonResult1, comparisonResult2, comparisonResult3],
    );
  }

  ShareClassComparisonSummary createFakeShareClassData() {
    final base1 = (
      isinCode: 'LU0808551500',
      mappedJson:
          '{"cod_isin":"LU0808551500","inst_scrnam":"Class EUR Shares","inst_scrsts":"Dormant","scrcry":"EUR"}',
    );

    final base2 = (
      isinCode: 'LU1227550453',
      mappedJson:
          '{"cod_isin":"LU1227550453","inst_scrnam":"Ordinary Shares","inst_scrsts":"Dormant","scrcry":"EUR"}',
    );

    final base3 = (
      isinCode: 'LU2407564710',
      mappedJson:
          '{"cod_isin":"LU2407564710","inst_scrnam":"Class A","inst_scrsts":"Dormant","scrcry":"USD"}',
    );

    final comparison1 = (
      isinCode: 'LU0808551500',
      mappedJson:
          '{"cod_isin":"LU0808551500","inst_scrnam":"Class EUR Shares","inst_scrsts":"Dormant","scrcry":"EUR"}',
    );

    final comparison2 = (
      isinCode: 'LU1227550453',
      mappedJson:
          '{"cod_isin":"LU1227550453","inst_scrnam":"Ordinary Shares","inst_scrsts":"Dormant","scrcry":"EUR"}',
    );

    final comparison3 = (
      isinCode: 'LU2407564710',
      mappedJson:
          '{"cod_isin":"LU2407564710","inst_scrnam":"Class A","inst_scrsts":"Dormant","scrcry":"USD"}',
    );

    return (
      dataType: "share class",
      base: [base1, base2, base3],
      comparison: [comparison1, comparison2, comparison3],
      result: [], // Empty list as requested
    );
  }
}

typedef ComparisonSummary = ({
  TopicComparisonSummary topicComparisonSummary,
  ShareClassComparisonSummary shareClassComparisonSummary,
});

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
