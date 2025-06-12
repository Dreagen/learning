import 'package:english_words/english_words.dart';
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
        theme: FlexThemeData.dark(scheme: FlexScheme.bigStone),
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
    Widget page;
    switch (selectedIndex) {
      case 0:
        page = MainPage();
      case 1:
        page = DetailsPage();
      default:
        throw UnimplementedError("no widget at index: $selectedIndex");
    }

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
                  child: page,
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
  List<ComparisonSummary> comparisonData = [];
  bool isLoading = true;

  @override
  void initState() {
    _loadData();
    super.initState();
  }

  Future<void> _loadData() async {
    final data = await repository.getItems();

    setState(() {
      comparisonData = data;
      isLoading = false;
    });
  }

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

        if (isLoading) {
          return Center(child: CircularProgressIndicator());
        }

        if (comparisonData.isEmpty) {
          return Center(child: Text("No comparison data available"));
        }

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
            children: comparisonData
                .map(
                  (run) => Card(
                    child: Padding(
                      padding: const EdgeInsets.all(20),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            "2025-06-10 10:36:00",
                            style: Theme.of(context).textTheme.titleLarge,
                          ),
                          const SizedBox(height: 15),
                          ConstrainedBox(
                            constraints: BoxConstraints(
                              minWidth: constraints.maxWidth - 60,
                            ),
                            child: DataTable(
                              columnSpacing: 24.0,
                              horizontalMargin: 12.0,
                              columns: [
                                DataColumn(label: Text('Data Type')),
                                DataColumn(
                                  label: Expanded(
                                    child: Text(
                                      'Base/Comparison Adds',
                                      textAlign: TextAlign.center,
                                    ),
                                  ),
                                ),
                                DataColumn(
                                  label: Expanded(
                                    child: Text(
                                      'Base/Comparison Mods',
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
                              rows: [
                                buildTopicDataRow(run.topicComparisonSummary),
                                buildShareClassDataRow(
                                  run.shareClassComparisonSummary,
                                ),
                              ],
                            ),
                          ),
                        ],
                      ),
                    ),
                  ),
                )
                .toList(),
          ),
        );
      },
    );
  }
}

DataRow buildTopicDataRow(TopicComparisonSummary topicComparisonSummary) {
  return DataRow(
    cells: [
      DataCell(Text(topicComparisonSummary.dataType)),
      DataCell(
        Center(
          child: Text(
            "+${topicComparisonSummary.base.where((x) => x.lastChangeType == "ADD").length.toString()} / +${topicComparisonSummary.comparison.where((x) => x.lastChangeType == "ADD").length.toString()}",
          ),
        ),
      ),
      DataCell(
        Center(
          child: Text(
            "~${topicComparisonSummary.base.where((x) => x.lastChangeType == "MOD").length.toString()} / ~${topicComparisonSummary.comparison.where((x) => x.lastChangeType == "MOD").length.toString()}",
          ),
        ),
      ),
      DataCell(
        Center(
          child: Text(
            topicComparisonSummary.result
                .where((x) => x.action == "ADD")
                .length
                .toString(),
            style: TextStyle(
              color:
                  topicComparisonSummary.result
                      .where((x) => x.action == "ADD")
                      .isNotEmpty
                  ? Colors.red
                  : Colors.green,
            ),
          ),
        ),
      ),
      DataCell(
        Center(
          child: Text(
            topicComparisonSummary.result
                .where((x) => x.action == "MOD")
                .length
                .toString(),
            style: TextStyle(
              color:
                  topicComparisonSummary.result
                      .where((x) => x.action == "MOD")
                      .isNotEmpty
                  ? Colors.red
                  : Colors.green,
            ),
          ),
        ),
      ),
      DataCell(
        Center(
          child: Text(
            topicComparisonSummary.result
                .where((x) => x.action == "DEL")
                .length
                .toString(),
            style: TextStyle(
              color:
                  topicComparisonSummary.result
                      .where((x) => x.action == "DEL")
                      .isNotEmpty
                  ? Colors.red
                  : Colors.green,
            ),
          ),
        ),
      ),
      DataCell(
        Center(
          child: topicComparisonSummary.result.isEmpty
              ? Icon(Icons.check, color: Colors.green)
              : Icon(Icons.error, color: Colors.red),
        ),
      ),
    ],
  );
}

DataRow buildShareClassDataRow(
  ShareClassComparisonSummary shareClassComparisonSummary,
) {
  return DataRow(
    cells: [
      DataCell(Text(shareClassComparisonSummary.dataType)),
      DataCell(Center(child: Text("N/A"))),
      DataCell(Center(child: Text("N/A"))),
      DataCell(
        Center(
          child: Text(
            shareClassComparisonSummary.result
                .where((x) => x.action == "ADD")
                .length
                .toString(),
            style: TextStyle(
              color:
                  shareClassComparisonSummary.result
                      .where((x) => x.action == "ADD")
                      .isNotEmpty
                  ? Colors.red
                  : Colors.green,
            ),
          ),
        ),
      ),
      DataCell(
        Center(
          child: Text(
            shareClassComparisonSummary.result
                .where((x) => x.action == "MOD")
                .length
                .toString(),
            style: TextStyle(
              color:
                  shareClassComparisonSummary.result
                      .where((x) => x.action == "MOD")
                      .isNotEmpty
                  ? Colors.red
                  : Colors.green,
            ),
          ),
        ),
      ),
      DataCell(
        Center(
          child: Text(
            shareClassComparisonSummary.result
                .where((x) => x.action == "DEL")
                .length
                .toString(),
            style: TextStyle(
              color:
                  shareClassComparisonSummary.result
                      .where((x) => x.action == "DEL")
                      .isNotEmpty
                  ? Colors.red
                  : Colors.green,
            ),
          ),
        ),
      ),
      DataCell(
        Center(
          child: shareClassComparisonSummary.result.isEmpty
              ? Icon(Icons.check, color: Colors.green)
              : Icon(Icons.error, color: Colors.red),
        ),
      ),
    ],
  );
}

class DetailsPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    var appState = context.watch<MyAppState>();

    return LayoutBuilder(
      builder: (context, constraints) {
        // Calculate dynamic padding based on available width
        final double horizontalPadding = constraints.maxWidth > 800
            ? 50.0
            : 20.0;
        final double verticalPadding = 20;

        // Calculate cards per row based on available space
        final int crossAxisCount = constraints.maxWidth > 1700 ? 2 : 1;

        if (appState.currentDetails == null) {
          return Center(child: Text("No comparison data available"));
        }

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
                          rows: appState
                              .currentDetails!
                              .topicComparisonSummary
                              .base
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
                          rows: appState
                              .currentDetails!
                              .topicComparisonSummary
                              .comparison
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
                          rows: appState
                              .currentDetails!
                              .topicComparisonSummary
                              .result
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
