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
  var current = WordPair.random();

  void getNext() {
    current = WordPair.random();
    notifyListeners();
  }

  var favourites = <WordPair>[];

  void toggleFavourite() {
    if (favourites.contains(current)) {
      favourites.remove(current);
    } else {
      favourites.add(current);
    }

    notifyListeners();
  }
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
        page = FavouritesPage();
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
        final int crossAxisCount = constraints.maxWidth > 1500 ? 2 : 1;

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
                          rows: comparisonData[0].topicComparisonSummary.base
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
                          rows: comparisonData[0]
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
                          rows: comparisonData[0].topicComparisonSummary.result
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

class FavouritesPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    var appState = context.watch<MyAppState>();

    if (appState.favourites.isEmpty) {
      return Center(child: Text("No favourites"));
    }

    return ListView(
      children: [
        Padding(
          padding: const EdgeInsets.all(20),
          child: Text("You have ${appState.favourites.length} favourites"),
        ),
        ...appState.favourites.map(
          (f) => ListTile(
            title: Text(f.asLowerCase),
            leading: Icon(Icons.favorite),
          ),
        ),
      ],
    );
  }
}
