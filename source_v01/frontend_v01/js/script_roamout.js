$(document).ready(function () {
    // Initialize Select2
    $('.select2').select2({
      theme: 'bootstrap4'
    });
  
    // Roamers by Country Donut Chart
    const donutCtx = document.getElementById('donutChart').getContext('2d');
    new Chart(donutCtx, {
      type: 'doughnut',
      data: {
        labels: ['USA', 'Germany', 'France'],
        datasets: [{
          data: [40, 35, 25],
          backgroundColor: ['#007bff', '#28a745', '#ffc107']
        }]
      },
      options: {
        maintainAspectRatio: false,
        responsive: true,
        plugins: {
          legend: { position: 'bottom' }
        }
      }
    });
  
    // Roamers by Country & Operator Stacked Bar Chart
    const roamersCtx = document.getElementById('roamers-chart').getContext('2d');
    new Chart(roamersCtx, {
      type: 'bar',
      data: {
        labels: ['Jan', 'Feb', 'Mar', 'Apr', 'May'],
        datasets: [
          {
            label: 'Operator A',
            data: [3000, 4000, 3500, 5000, 4500],
            backgroundColor: '#007bff'
          },
          {
            label: 'Operator B',
            data: [2000, 3000, 2500, 4000, 3500],
            backgroundColor: '#28a745'
          },
          {
            label: 'Operator C',
            data: [1500, 2000, 1800, 2500, 2200],
            backgroundColor: '#ffc107'
          }
        ]
      },
      options: {
        maintainAspectRatio: false,
        responsive: true,
        plugins: {
          legend: { position: 'bottom' },
          title: {
            display: true,
            text: 'Roamers by Country & Operator'
          }
        },
        scales: {
          x: {
            stacked: true
          },
          y: {
            stacked: true,
            beginAtZero: true
          }
        }
      }
    });
  });
  