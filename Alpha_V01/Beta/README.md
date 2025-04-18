GET /api/analytics/overview
{
  "total_sales": 152000,
  "total_orders": 1340,
  "average_order_value": 113.43,
  "start_date": "2024-01-01",
  "end_date": "2025-04-17"
}

GET /api/analytics/sales-over-time?interval=monthly&start=2024-01-01&end=2024-12-31
[
  { "period": "2024-01", "sales": 12000 },
  { "period": "2024-02", "sales": 15000 },
  ...
]


GET /api/analytics/sales-by-product?start=2024-01-01&end=2024-12-31
[
  { "product_id": "P001", "product_name": "Laptop", "sales": 54000 },
  { "product_id": "P002", "product_name": "Headphones", "sales": 13000 }
]

GET /api/analytics/sales-by-region?start=2024-01-01&end=2024-12-31
[
  { "region": "North America", "sales": 74000 },
  { "region": "Europe", "sales": 42000 }
]

GET /api/analytics/top-customers?limit=10
[
  { "customer_id": "C001", "name": "Alice", "total_spent": 6700 },
  { "customer_id": "C002", "name": "Bob", "total_spent": 6200 }
]

Filtering
GET /api/analytics/sales-by-product?category=Electronics&channel=Online&salesperson_id=123


export CSV
GET /api/analytics/sales-by-region/export?format=csv

drill-down
GET /api/analytics/sales-by-product/P001







http-server . -p 8080


project/
├── index.html           # The file above
├── dist/                # Contains AdminLTE files
│   ├── css/             # adminlte.min.css
│   └── js/              # adminlte.min.js
├── plugins/             # Contains plugins (jquery, moment, daterangepicker)
│   ├── jquery/          # jquery.min.js
│   ├── moment/          # moment.min.js
│   └── daterangepicker/ # daterangepicker.js


