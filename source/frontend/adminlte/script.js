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

    const ctx = document.getElementById('roamOutDatesChart').getContext('2d');
    new Chart(ctx, {
      type: 'line',
      data: {
        labels: dates,
        datasets: [{
          label: 'Roam Out Counts',
          data: counts,
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

// Fetch notifications
async function fetchNotifications() {
  try {
    const response = await fetch('http://localhost:3000/api/v1/stats/notifications');
    if (!response.ok) throw new Error(`HTTP error! status: ${response.status}`);
    const result = await response.json();
    const notifications = result.data;

    const calloutsContainer = document.getElementById('notifications-list');
    calloutsContainer.innerHTML = '';

    notifications.forEach(item => {
      const callout = document.createElement('div');
      callout.classList.add('callout', 'callout-danger');
      const message = document.createElement('p');
      message.textContent = item.notification;
      callout.appendChild(message);
      calloutsContainer.appendChild(callout);
    });

  } catch (error) {
    console.error('Failed to fetch notifications:', error);
  }
}


fetch('http://localhost:3000/api/v1/stats/roamout_countries')
.then(response => response.json())
.then(result => {
  const data = result.data;
  const top = data.slice(0, 10);
  const countryLabels = top.map(item => item.country);
  const counts = top.map(item => item.count);
  const total = counts.reduce((sum, count) => sum + count, 0);

  const colors = [
    '#007bff', '#28a745', '#ffc107', '#dc3545',
    '#17a2b8', '#6f42c1', '#e83e8c', '#fd7e14',
    '#20c997', '#6610f2'
  ];

  const ctx = document.getElementById('roamOutDonutChart').getContext('2d');
  new Chart(ctx, {
    type: 'doughnut',
    data: {
      labels: countryLabels,
      datasets: [{
        data: counts,
        backgroundColor: colors
      }]
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      plugins: {
        legend: { display: false },
        tooltip: {
          callbacks: {
            label: function(context) {
              const value = context.parsed;
              const percent = ((value / total) * 100).toFixed(1);
              return `${context.label}: ${value} (${percent}%)`;
            }
          }
        }
      }
    }
  });

  // Add legend manually below the chart
  const legendList = document.getElementById('roamOutLegend');
  legendList.innerHTML = ''; // clear previous
  top.forEach((item, index) => {
    const percent = ((item.count / total) * 100).toFixed(1);
    const li = document.createElement('li');
    li.classList.add('nav-item');

    li.innerHTML = `
      <a href="#" class="nav-link">
        <i class="fas fa-circle mr-2" style="color:${colors[index]}"></i> 
        ${item.country}
        <span class="float-right text-muted">${percent}%</span>
      </a>
    `;
    legendList.appendChild(li);
  });
})
.catch(err => console.error('Failed to fetch donut data', err));

fetch('http://localhost:3000/api/v1/stats/roamout_operators')
  .then(response => response.json())
  .then(result => {
    const data = result.data;
    const tbody = document.getElementById('operator-table-body');
    const dateSpan = document.getElementById('operator-data-date');
    tbody.innerHTML = '';

    // Sort by count descending
    data.sort((a, b) => b.count - a.count);

    // Get latest date
    const latestDate = data.length > 0 ? data[0].date : 'N/A';
    dateSpan.textContent = latestDate;

    data.forEach(item => {
      const tr = document.createElement('tr');
      tr.innerHTML = `
        <td><strong>${item.operator}</strong></td>
        <td>${item.country}</td>
        <td><span class="badge badge-primary">${item.count}</span></td>
        <td>${item.date}</td>
      `;
      tbody.appendChild(tr);
    });
  })
  .catch(err => console.error('Failed to fetch roamout_operators data:', err));


// Fetching anomalies data
fetch('http://localhost:3000/api/v1/stats/anomalies')
  .then(response => response.json())
  .then(data => {
    const anomalies = data.data;
    const tableBody = document.getElementById('anomalies-table').getElementsByTagName('tbody')[0];

    anomalies.forEach(anomaly => {
      const row = tableBody.insertRow();
      row.innerHTML = `
        <td>${anomaly.name_en}</td>
        <td>${anomaly.operator}</td>
        <td>${anomaly.configure}</td>
        <td>${anomaly.reel}</td>
        <td>${anomaly.country_count}</td>
        <td>${anomaly.operator_count}</td>
        <td>${anomaly.routage}</td>
      `;
    });
  })
  .catch(err => console.error('Failed to fetch anomalies data', err));


// Initialize all fetch functions
window.onload = () => {
  fetchNotifications();
  fetchRoamOutDonut();
};
