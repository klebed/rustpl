{% if proto in [ "udp", "tcp", "tls" ] %}
listen {
    virtual_server = {{server}}
    type           = auth+acct
    ipaddr         = *
    port           = {{port}}
{% if proto == "udp" %}
    proto          = udp
{% elif proto == "tcp" %}
    proto          = tcp
{# ToDo check limits (avoid tcp-conn warnings in log) #}
    limit {
        lifetime     = 0
        idle_timeout = 600
    }
{% elif proto == "tls" %}
    proto          = tcp
    transport      = tls
{#    tls=${..tls} #}
    tls {
         certificate_file     = {{ tls_certificate_file }}
         private_key_file     = {{ tls_private_key_file }}
         private_key_password = {{ tls_private_key_password }}
     }
{% endif %}
}
{% else %}
# !error: unknown proto '{{proto}}'
{% endif %}



def_default={{ tls_private_key_password | default (value="x") }}
def_undef={% if tls_private_key_password is defined %}{{ tls_private_key_password }}{% else %}x{% endif %}
def_if={% if tls_private_key_password %}{{ tls_private_key_password }}{% else %}x{% endif %}
