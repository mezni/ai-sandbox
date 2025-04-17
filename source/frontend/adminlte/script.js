// Fetching overview data
fetch('http://localhost:3000/api/v1/stats/overview')
  .then(response => response.json())
  .then(data => {
    const d = data.data;
    document.getElementById('last_date').textContent = d.last_date;
    document.getElementById('count_roam_in').textContent = d.count_roam_in;
    document.getElementById('count_roam_out').textContent = d.count_roam_out;
    document.getElementById('count_anomalies').textContent = d.count_anomalies;
    document.getElementById('count_notifications').textContent = d.count_notifications;
  })
  .catch(err => console.error('Failed to fetch overview data', err));

// Fetching Roam Out Dates data for chart
fetch('http://localhost:3000/api/v1/stats/roamout_dates')
  .then(response => response.json())
  .then(data => {
    const dates = data.data.map(item => item.date);
    const counts = data.data.map(item => item.count);

    // Create chart
    const ctx = document.getElementById('roamOutDatesChart').getContext('2d');
    const roamOutDatesChart = new Chart(ctx, {
      type: 'line',
      data: {
        labels: dates, // Dates as labels
        datasets: [{
          label: 'Roam Out Counts',
          data: counts, // Roam out counts
          borderColor: '#28a745',
          backgroundColor: 'rgba(40, 167, 69, 0.2)',
          fill: true
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        scales: {
          x: {
            title: {
              display: true,
              text: 'Date'
            }
          },
          y: {
            title: {
              display: true,
              text: 'Count'
            },
            beginAtZero: true
          }
        }
      }
    });
  })
  .catch(err => console.error('Failed to fetch roam out dates data', err));
