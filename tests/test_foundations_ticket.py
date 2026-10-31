from ccaf.foundations.ticket import Ticket, prepare_classification_request


def test_prepare_classification_request():
    ticket = Ticket("T-001", "My internet connection has stopped working.")
    request = prepare_classification_request(ticket)
    assert ticket.ticket_id == "T-001"
    assert ticket.message == "My internet connection has stopped working."
    assert "technical_support" in request["categories"]
