// Function to handle 'View' action
function viewCountry(isoCode) {
    alert('Viewing details for country with ISO: ' + isoCode);
    // Implement your logic here
  }
  
  // Function to handle 'Edit' action
  function editCountry(isoCode) {
    alert('Editing country with ISO: ' + isoCode);
    // Implement your logic here
  }
  
  // Function to handle 'Delete' action
  function deleteCountry(isoCode) {
    if (confirm('Are you sure you want to delete this country?')) {
      alert('Deleted country with ISO: ' + isoCode);
      // Implement your logic here
    }
  }
  