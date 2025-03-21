import phonenumbers
from phonenumbers import carrier, geocoder, timezone

def process_phone_number(phone_number, region_code=None):
    try:
        # Parse the phone number
        parsed_number = phonenumbers.parse(phone_number, region_code)

        # Validate the phone number
        if not phonenumbers.is_valid_number(parsed_number):
            return "Invalid phone number."

        # Format the phone number
        formatted_international = phonenumbers.format_number(parsed_number, phonenumbers.PhoneNumberFormat.INTERNATIONAL)
        formatted_e164 = phonenumbers.format_number(parsed_number, phonenumbers.PhoneNumberFormat.E164)

        # Get region and carrier information
        region = phonenumbers.region_code_for_number(parsed_number)
        carrier_name = carrier.name_for_number(parsed_number, 'en')
        location = geocoder.description_for_number(parsed_number, 'en')
        time_zones = timezone.time_zones_for_number(parsed_number)

        return {
            "valid": True,
            "formatted_international": formatted_international,
            "formatted_e164": formatted_e164,
            "region": region,
            "carrier": carrier_name,
            "location": location,
            "time_zones": time_zones,
        }
    except phonenumbers.phonenumberutil.NumberParseException as e:
        return f"Error parsing phone number: {e}"

# Example usage
phone_number = "+33761443850"  # Replace with any phone number
result = process_phone_number(phone_number)
print(result)
phone_number = "+16472193624"  # Replace with any phone number
result = process_phone_number(phone_number)
print(result)
