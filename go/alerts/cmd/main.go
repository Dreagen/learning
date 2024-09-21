package main

import (
	"html/template"
	"net/http"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
)

type Alert struct {
	Name string
	Date string
}

type PageData struct {
	Alerts []Alert
}

func main() {
	router := chi.NewRouter()
	router.Use(middleware.Logger)

	router.Get("/", func(writer http.ResponseWriter, request *http.Request) {
		alerts := []Alert{
			{Name: "Alert1", Date: "2024-09-21"},
			{Name: "Alert2", Date: "2024-09-22"},
		}

		data := PageData{
			Alerts: alerts,
		}

		tmpl := template.Must(template.ParseFiles("views/index.html", "views/alerts_view_all.html"))
		tmpl.ExecuteTemplate(writer, "index", data)
	})

	http.ListenAndServe(":3000", router)
}
