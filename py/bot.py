from bottle import route, run, redirect, request, response
import RPi.GPIO as GPIO
import time
import sys
import os

def set_angle(angle, servo):
    duty = angle / 18 + 2
    servo.ChangeDutyCycle(duty)
    time.sleep(0.5)
    servo.ChangeDutyCycle(0)

@route('/press')
def press():
    e = ''
    try:
        GPIO.setmode(GPIO.BOARD)
        OUTPUT_PIN = 11
        GPIO.setup(OUTPUT_PIN, GPIO.OUT)

        servo = GPIO.PWM(OUTPUT_PIN, 50)
        servo.start(0)
        set_angle(180.0, servo)

        time.sleep(2)
        set_angle(90.0, servo)

        servo.stop()
    except KeyboardInterrupt:
        print('Exiting...')
    except Exception as err:
        print('ERROR:', err)
        e = 'true'
    finally:
        GPIO.cleanup()
        url = '/'
        if e != '':
            url += '?e={}'.format(e)
        return redirect(url)

@route('/')
def index():
    q = request.query.get('e', '')
    err = '''<p>Something weird happened. Try again.''' if q == 'true' else ''

    return '''
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale="1.0">
            <style>
                body {{ min-height: 100vh; display: flex; flex-direction: column; justify-content: center; align-items: center; }}
                #submit {{ box-shadow: inset 0 0 4px 0px #FFFFFF; background-color: #8E24AA; color: #ffffff; font-size: 2.5em; border: 1px solid #FFFFFF; border-radius: 6px; padding: 6px 24px; }}
                #submit:disabled {{ background-color: gray; }}
            </style>
        </head>
        <body>
            <form action="/press" onsubmit="document.getElementById('submit').disabled=true">
                <input type="submit" id="submit" value="افتح يا سمسم"></input>
            </form>
            {}
        </body>
        </html>
    '''.format(err)

if __name__ == "__main__":
    debug = True if os.environ.get('DEBUG') else False
    if debug:
        print("Starting in debug mode!")
    run(host='0.0.0.0', port=80, debug=debug, reloader=debug)
