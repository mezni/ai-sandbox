document.addEventListener('DOMContentLoaded', function () {
  fetch('http://localhost:3000/api/v1/notifications/details')
    .then(response => response.json())
    .then(data => {
      const tableBody = document.querySelector('#performance-table tbody');
      tableBody.innerHTML = ''; // Clear existing rows if any

      data.data.forEach(item => {
        const row = document.createElement('tr');
        row.innerHTML = `
          <td>${item.operator}</td>
          <td>${item.perct_configure}%</td>
          <td>${item.perct_reel}%</td>
        `;
        tableBody.appendChild(row);
      });
    })
    .catch(error => {
      console.error('Error loading data:', error);
    });
});

