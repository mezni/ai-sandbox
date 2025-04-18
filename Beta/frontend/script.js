$(function () {
    $('#daterange').daterangepicker({
      opens: 'right',
      autoUpdateInput: false,
      locale: {
        cancelLabel: 'Clear'
      }
    });
  
    $('#daterange').on('apply.daterangepicker', function(ev, picker) {
      const start = picker.startDate.format('YYYY-MM-DD');
      const end = picker.endDate.format('YYYY-MM-DD');
      $(this).val(`${start} - ${end}`);
      console.log('Selected:', start, 'to', end);
    });
  
    $('#daterange').on('cancel.daterangepicker', function(ev, picker) {
      $(this).val('');
    });
  });
  