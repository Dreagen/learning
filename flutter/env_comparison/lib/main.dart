import 'package:english_words/english_words.dart';
import 'package:flex_color_scheme/flex_color_scheme.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

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

class MainPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(
      builder: (context, constraints) {
        // Calculate dynamic padding based on available width
        final double horizontalPadding = constraints.maxWidth > 800
            ? 50.0
            : 20.0;
        final double verticalPadding = constraints.maxHeight > 600
            ? 50.0
            : 20.0;

        // Calculate cards per row based on available space
        final int crossAxisCount = constraints.maxWidth > 600 ? 1 : 1;

        return Padding(
          padding: EdgeInsets.only(
            left: horizontalPadding,
            top: verticalPadding,
            right: horizontalPadding,
            bottom: verticalPadding,
          ),
          child: GridView.count(
            crossAxisCount: crossAxisCount,
            childAspectRatio: 2.5,
            mainAxisSpacing: 10,
            crossAxisSpacing: 10,
            children: [
              Card(
                child: Padding(
                  padding: const EdgeInsets.all(20),
                  child: Text("Card 1"),
                ),
              ),
              Card(
                child: Padding(
                  padding: const EdgeInsets.all(20),
                  child: Text("Card 2"),
                ),
              ),
              Card(
                child: Padding(
                  padding: const EdgeInsets.all(20),
                  child: Text("Card 3"),
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
