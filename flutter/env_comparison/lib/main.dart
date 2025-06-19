import 'package:english_words/english_words.dart';
import 'package:env_comparison/data_mapper.dart';
import 'package:env_comparison/topics/topic_comparison_summary.dart';
import 'package:env_comparison/utils/colors.dart';
import 'package:env_comparison/widgets/result_chart.dart';
import 'package:flex_color_scheme/flex_color_scheme.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import 'repository.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return ChangeNotifierProvider(
      create: (context) => MyAppState(),
      child: MaterialApp(
        title: 'Environment Comparison',
        theme: FlexThemeData.dark(scheme: FlexScheme.materialBaseline),
        home: MyHomePage(),
      ),
    );
  }
}

class MyAppState extends ChangeNotifier {
  ComparisonSummary? currentDetails;
}

class MyHomePage extends StatefulWidget {
  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  var selectedIndex = 0;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        return Scaffold(
          appBar: AppBar(
            title: Text("Environment comparison Development | Test"),
            actions: <Widget>[Icon(Icons.code)],
          ),
          body: Row(
            children: [
              Expanded(
                child: Container(
                  color: Theme.of(context).colorScheme.primary,
                  child: MainPage(),
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}

class MainPage extends StatefulWidget {
  @override
  State<MainPage> createState() => _MainPageState();
}

class _MainPageState extends State<MainPage> {
  final Repository repository = Repository();
  List<ComparisonSummaryForTable> tableData = [];
  DateTime? runTime;
  bool isLoading = true;

  @override
  void initState() {
    _loadData();
    super.initState();
  }

  Future<void> _loadData() async {
    final data = await repository.getLatest();

    setState(() {
      tableData = DataMapper().mapFromComparisonSummary(data);
      runTime = data.runTime;
      isLoading = false;

      print("runTime $runTime");
    });
  }

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        if (isLoading) {
          return Center(child: CircularProgressIndicator());
        }

        if (tableData.isEmpty) {
          return Center(child: Text("No comparison data available"));
        }

        return Padding(
          padding: EdgeInsets.all(20),
          child: Card(
            elevation: 20,
            child: Padding(
              padding: const EdgeInsets.all(20),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    runTime.toString(),
                    // "testing",
                    style: Theme.of(context).textTheme.titleLarge,
                  ),
                  const SizedBox(height: 15),
                  Row(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Expanded(
                        flex: 4,
                        child: ConstrainedBox(
                          constraints: BoxConstraints(
                            minWidth: 0.75 * (constraints.maxWidth - 60),
                          ),
                          child: DataTable(
                            columnSpacing: 24.0,
                            horizontalMargin: 12.0,
                            columns: [
                              DataColumn(label: Text('Data Type')),
                              DataColumn(
                                label: Expanded(
                                  child: Text(
                                    'Base Changes',
                                    textAlign: TextAlign.center,
                                  ),
                                ),
                              ),
                              DataColumn(
                                label: Expanded(
                                  child: Text(
                                    'Comparison Changes',
                                    textAlign: TextAlign.center,
                                  ),
                                ),
                              ),
                              DataColumn(
                                label: Expanded(
                                  child: Text(
                                    'Result Adds',
                                    textAlign: TextAlign.center,
                                  ),
                                ),
                              ),
                              DataColumn(
                                label: Expanded(
                                  child: Text(
                                    'Result Mods',
                                    textAlign: TextAlign.center,
                                  ),
                                ),
                              ),
                              DataColumn(
                                label: Expanded(
                                  child: Text(
                                    'Result Dels',
                                    textAlign: TextAlign.center,
                                  ),
                                ),
                              ),
                              DataColumn(label: Text('')),
                            ],
                            rows: tableData.map((td) {
                              return buildDataRow(context, td);
                            }).toList(),
                          ),
                        ),
                      ),
                      Expanded(flex: 2, child: ResultChart()),
                    ],
                  ),
                ],
              ),
            ),
          ),
        );
      },
    );
  }
}

typedef ComparisonSummaryForTable = ({
  String dataType,
  int baseChanges,
  int comparisonChanges,
  int resultAdds,
  int resultMods,
  int resultDels,
});

extension ComparisonSummaryExtension on ComparisonSummaryForTable {
  bool get isMatching => resultAdds == 0 && resultMods == 0 && resultDels == 0;
}

DataRow buildDataRow(
  BuildContext context,
  ComparisonSummaryForTable comparisonSummary,
) {
  return DataRow(
    onSelectChanged: (_) => {},
    cells: [
      DataCell(Text(comparisonSummary.dataType)),
      DataCell(Center(child: Text(comparisonSummary.baseChanges.toString()))),
      DataCell(
        Center(child: Text(comparisonSummary.comparisonChanges.toString())),
      ),
      DataCell(
        Center(
          child: Text(
            comparisonSummary.resultAdds.toString(),
            style: TextStyle(
              color: comparisonSummary.resultAdds > 0
                  ? AppColors.add
                  : Colors.white,
            ),
          ),
        ),
      ),
      DataCell(
        Center(
          child: Text(
            comparisonSummary.resultMods.toString(),
            style: TextStyle(
              color: comparisonSummary.resultMods > 0
                  ? AppColors.mod
                  : Colors.white,
            ),
          ),
        ),
      ),
      DataCell(
        Center(
          child: Text(
            comparisonSummary.resultDels.toString(),
            style: TextStyle(
              color: comparisonSummary.resultDels > 0
                  ? AppColors.del
                  : Colors.white,
            ),
          ),
        ),
      ),
      DataCell(
        Center(
          child: comparisonSummary.isMatching
              ? Icon(Icons.check, color: Colors.green)
              : Icon(Icons.error, color: Colors.red),
        ),
      ),
    ],
  );
}

class TopicDetailsPage extends StatelessWidget {
  final TopicComparisonSummary topicComparisonSummary;
  const TopicDetailsPage({super.key, required this.topicComparisonSummary});

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        // Calculate dynamic padding based on available width
        final double horizontalPadding = constraints.maxWidth > 800
            ? 50.0
            : 20.0;
        final double verticalPadding = 20;

        // Calculate cards per row based on available space
        final int crossAxisCount = constraints.maxWidth > 1700 ? 2 : 1;

        return Padding(
          padding: EdgeInsets.only(
            left: horizontalPadding,
            top: verticalPadding,
            right: horizontalPadding,
            bottom: verticalPadding,
          ),
          child: GridView.count(
            crossAxisCount: crossAxisCount,
            childAspectRatio: 2,
            mainAxisSpacing: 10,
            crossAxisSpacing: 10,
            children: [
              Card(
                child: Padding(
                  padding: const EdgeInsets.all(20),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        "BASE",
                        style: Theme.of(context).textTheme.titleLarge,
                      ),
                      const SizedBox(height: 15),
                      ConstrainedBox(
                        constraints: BoxConstraints(
                          minWidth:
                              constraints.maxWidth -
                              60, // Card padding (20) × 2 + some buffer
                        ),
                        child: DataTable(
                          columnSpacing: 24.0,
                          horizontalMargin: 12.0,
                          columns: const [
                            DataColumn(label: Text('ISIN')),
                            DataColumn(label: Text('Currency')),
                            DataColumn(label: Text('Status')),
                            DataColumn(label: Text('Last Change')),
                            DataColumn(label: Text('Processed')),
                          ],
                          rows: topicComparisonSummary.base
                              .map(
                                (item) => DataRow(
                                  cells: [
                                    DataCell(Text(item.isinCode)),
                                    DataCell(Text(item.currency)),
                                    DataCell(Text(item.status)),
                                    DataCell(Text(item.lastChangeType)),
                                    DataCell(Text(item.lastChangeProcessed)),
                                  ],
                                ),
                              )
                              .toList(),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
              Card(
                child: Padding(
                  padding: const EdgeInsets.all(20),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        "COMPARISON",
                        style: Theme.of(context).textTheme.titleLarge,
                      ),
                      const SizedBox(height: 15),
                      ConstrainedBox(
                        constraints: BoxConstraints(
                          minWidth:
                              constraints.maxWidth -
                              60, // Card padding (20) × 2 + some buffer
                        ),
                        child: DataTable(
                          columnSpacing: 24.0,
                          horizontalMargin: 12.0,
                          columns: const [
                            DataColumn(label: Text('ISIN')),
                            DataColumn(label: Text('Currency')),
                            DataColumn(label: Text('Status')),
                            DataColumn(label: Text('Last Change')),
                            DataColumn(label: Text('Processed')),
                          ],
                          rows: topicComparisonSummary.comparison
                              .map(
                                (item) => DataRow(
                                  cells: [
                                    DataCell(Text(item.isinCode)),
                                    DataCell(Text(item.currency)),
                                    DataCell(Text(item.status)),
                                    DataCell(Text(item.lastChangeType)),
                                    DataCell(Text(item.lastChangeProcessed)),
                                  ],
                                ),
                              )
                              .toList(),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
              Card(
                child: Padding(
                  padding: const EdgeInsets.all(20),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        "RESULTS",
                        style: Theme.of(context).textTheme.titleLarge,
                      ),
                      const SizedBox(height: 15),
                      ConstrainedBox(
                        constraints: BoxConstraints(
                          minWidth:
                              constraints.maxWidth -
                              60, // Card padding (20) × 2 + some buffer
                        ),
                        child: DataTable(
                          columns: const [
                            DataColumn(label: Text('Action')),
                            DataColumn(label: Text('ISIN')),
                            DataColumn(label: Text('Currency')),
                            DataColumn(label: Text('Change')),
                          ],
                          rows: topicComparisonSummary.result
                              .map(
                                (item) => DataRow(
                                  cells: [
                                    DataCell(Text(item.action)),
                                    DataCell(Text(item.isinCode)),
                                    DataCell(Text(item.currency)),
                                    DataCell(Text(item.lastChangeProcessed)),
                                  ],
                                ),
                              )
                              .toList(),
                        ),
                      ),
                    ],
                  ),
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}

class BigCard extends StatelessWidget {
  const BigCard({super.key, required this.pair});

  final WordPair pair;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final style = theme.textTheme.displayMedium!.copyWith(
      color: theme.colorScheme.onPrimary,
    );

    return Card(
      color: theme.colorScheme.primary,
      elevation: 5,
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Text(
          pair.asLowerCase,
          style: style,
          semanticsLabel: "${pair.first}, ${pair.second}",
        ),
      ),
    );
  }
}
