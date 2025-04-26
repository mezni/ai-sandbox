document.addEventListener('DOMContentLoaded', function() {
    fetchRoamOutMetrics();
    fetchRoamInMetrics();
    
    // Optionally refresh every 5 minutes
    setInterval(fetchRoamOutMetrics, 300000);
    setInterval(fetchRoamInMetrics, 300000);
});

function fetchRoamOutMetrics() {
    fetch("http://localhost:3000/api/v1/metrics?direction=out&dimensions=global")
      .then(response => {
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        return response.json();
      })
      .then(data => {
        if (data.data && data.data.length > 0) {
          const metrics = data.data[0]; // Get the first item in the array
          document.getElementById('roam-out-count').textContent = metrics.count.toLocaleString();
          document.getElementById('roam-out-date').textContent = metrics.date;
        }
      })
      .catch(error => {
        console.error('Error fetching metrics:', error);
        document.getElementById('roam-out-count').textContent = 'Error';
        document.getElementById('roam-out-date').textContent = '';
      });
}

function fetchRoamInMetrics() {
    fetch("http://localhost:3000/api/v1/metrics?direction=totin&dimensions=global")
      .then(response => {
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        return response.json();
      })
      .then(data => {
        if (data.data && data.data.length > 0) {
          const metrics = data.data[0]; // Get the first item in the array
          document.getElementById('roam-in-count').textContent = metrics.count.toLocaleString();
          document.getElementById('roam-in-date').textContent = metrics.date;
        }
      })
      .catch(error => {
        console.error('Error fetching metrics:', error);
        document.getElementById('roam-in-count').textContent = 'Error';
        document.getElementById('roam-in-date').textContent = '';
      });
}
