import 'package:env_comparison/share_classes/share_class_comparison_summary.dart';
import 'package:env_comparison/topics/topic_comparison_summary.dart';

import 'media_outlets/media_outlet_map_comparison_summary.dart';

class Repository {
  Future<ComparisonSummary> getLatest() async {
    var topicData = createFakeTopicComparisonData();
    var shareClassData = createFakeShareClassComparisonData();
    var mediaOutletMapData = createFakeMediaOutletMapDataComparisonData();

    return (
      runTime: DateTime.now().add(const Duration(seconds: -1)),
      topicComparisonSummary: topicData,
      shareClassComparisonSummary: shareClassData,
      mediaOutletMapComparisonSummary: mediaOutletMapData,
    );
  }

  TopicComparisonSummary createFakeTopicComparisonData() {
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

    final comparisonModResult1 = (
      action: 'MOD',
      isinCode: 'LU1227550453',
      currency: 'EUR',
      mappedJson: '{"pubcry":"EUR"}',
      lastChangeType: 'ADD',
      lastChangeProcessed: 'DIFF(true->false)',
    );

    final comparisonModResult2 = (
      action: 'MOD',
      isinCode: 'LU2407564710',
      currency: 'USD',
      mappedJson: '{"pubcry":"USD"}',
      lastChangeType: 'ADD',
      lastChangeProcessed: 'DIFF(true->false)',
    );

    final comparisonModResult3 = (
      action: 'MOD',
      isinCode: 'LU0808551500',
      currency: 'EUR',
      mappedJson: '{"pubcry":"EUR"}',
      lastChangeType: 'ADD',
      lastChangeProcessed: 'DIFF(true->false)',
    );

    return (
      dataType: "Topic",
      base: [base1, base2, base3],
      comparison: [comparison1, comparison2, comparison3],
      result: [
        comparisonModResult1,
        comparisonModResult2,
        comparisonModResult3,
      ],
    );
  }

  ShareClassComparisonSummary createFakeShareClassComparisonData() {
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
      dataType: "Share Class",
      base: [base1, base2, base3],
      comparison: [comparison1, comparison2, comparison3],
      result: [],
    );
  }

  MediaOutletMapComparisonSummary createFakeMediaOutletMapDataComparisonData() {
    final MediaOutletMapComparison base1 = (
      mediaOutletName: 'Morningstar',
      isinCode: 'LU2407564710',
      currency: 'USD',
      status: 'Active',
      cancellationDate: null,
      cancellationReason: null,
    );

    final base2 = (
      mediaOutletName: 'WertpapierMitteilungen',
      isinCode: 'LU0808551500',
      currency: 'EUR',
      status: 'Active',
      cancellationDate: null,
      cancellationReason: null,
    );

    final base3 = (
      mediaOutletName: 'Morningstar',
      isinCode: 'LU1227550453',
      currency: 'EUR',
      status: 'Active',
      cancellationDate: null,
      cancellationReason: null,
    );

    final base4 = (
      mediaOutletName: 'Morningstar',
      isinCode: 'LU0808551500',
      currency: 'EUR',
      status: 'Active',
      cancellationDate: null,
      cancellationReason: null,
    );

    final base5 = (
      mediaOutletName: 'WertpapierMitteilungen',
      isinCode: 'LU2407564710',
      currency: 'USD',
      status: 'Active',
      cancellationDate: null,
      cancellationReason: null,
    );

    final base6 = (
      mediaOutletName: 'WertpapierMitteilungen',
      isinCode: 'LU1227550453',
      currency: 'EUR',
      status: 'Active',
      cancellationDate: null,
      cancellationReason: null,
    );

    final comparisonResult1 = (
      action: 'DEL',
      mediaOutletName: 'Morningstar',
      isinCode: 'LU1227550453',
      currency: 'EUR',
      status: 'Active',
      cancellationDate: null,
      cancellationReason: null,
    );

    final comparisonResult2 = (
      action: 'DEL',
      mediaOutletName: 'Morningstar',
      isinCode: 'LU0808551500',
      currency: 'EUR',
      status: 'Active',
      cancellationDate: null,
      cancellationReason: null,
    );

    final comparisonResult3 = (
      action: 'DEL',
      mediaOutletName: 'WertpapierMitteilungen',
      isinCode: 'LU0808551500',
      currency: 'EUR',
      status: 'Active',
      cancellationDate: null,
      cancellationReason: null,
    );

    final comparisonResult4 = (
      action: 'DEL',
      mediaOutletName: 'WertpapierMitteilungen',
      isinCode: 'LU2407564710',
      currency: 'USD',
      status: 'Active',
      cancellationDate: null,
      cancellationReason: null,
    );

    final comparisonResult5 = (
      action: 'DEL',
      mediaOutletName: 'WertpapierMitteilungen',
      isinCode: 'LU1227550453',
      currency: 'EUR',
      status: 'Active',
      cancellationDate: null,
      cancellationReason: null,
    );

    final comparisonResult6 = (
      action: 'DEL',
      mediaOutletName: 'Morningstar',
      isinCode: 'LU2407564710',
      currency: 'USD',
      status: 'Active',
      cancellationDate: null,
      cancellationReason: null,
    );

    return (
      dataType: "Media Outlet Map",
      base: [base1, base2, base3, base4, base5, base6],
      comparison: [],
      result: [
        comparisonResult1,
        comparisonResult2,
        comparisonResult3,
        comparisonResult4,
        comparisonResult5,
        comparisonResult6,
      ],
    );
  }
}

typedef ComparisonSummary = ({
  DateTime runTime,
  TopicComparisonSummary topicComparisonSummary,
  ShareClassComparisonSummary shareClassComparisonSummary,
  MediaOutletMapComparisonSummary mediaOutletMapComparisonSummary,
});
