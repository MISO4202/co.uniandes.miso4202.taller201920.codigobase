var mockaroo = require('mockaroo');
var fs = require('fs');
var json2xls = require('json2xls');

var client = new mockaroo.Client({
        apiKey: '348d9a60'
    });

var country = client.generate({
        count: 5,
        format: 'json',
        header: true,
        fields: [{
            name: 'id',
            type: 'Row Number'
        }, 
        {
            name: 'shortName',
            type: 'Country Code'
        },
        {
            name: 'name',
            type: 'Country'
        },
        {
            name: 'native',
            type: 'Words',
            min:1,
            max:1
        },
        {
            name: 'phone',
            type: 'Phone'
        },
        {
            name: 'continent',
            type: 'Words',
            min:1,
            max:1
        },
        {
            name: 'capital',
            type: 'City'
        },
        {
            name: 'currency',
            type: 'Words',
            min:1,
            max:1
        },
        {
            name: 'languages',
            type: 'Words',
            min:1,
            max:1
        },
        {
            name: 'emoji',
            type: 'Words',
            min:1,
            max:1
        },
        {
            name: 'emojiU',
            type: 'Words',
            min:1,
            max:1
        }]
    });

    var student = client.generate({
        count: 5,
        format: 'json',
        header: true,
        fields: [{
            name: 'id',
            type: 'Row Number'
        }, 
        {
            name: 'first_name',
            type: 'First Name'
        }, 
        {
            name: 'last_name',
            type: 'Last Name'
        }, 
        {
            name: 'gender',
            type: 'Gender'
        }, 
        {
            name: 'code',
            type: 'EIN'
        }, 
        {
            name: 'country',
            type: 'Country'
        }, 
        {
            name: 'city',
            type: 'City'
        }, 
        {
            name: 'university',
            type: 'University'
        }]
    });

    
    country.then(function(countries) {
        var countriesxls = json2xls(countries);
        fs.writeFileSync('./data/countries.xlsx', countriesxls, 'binary');
    });

    student.then(function(students) {
        var studentsxls = json2xls(students);
        fs.writeFileSync('./data/students.xlsx', studentsxls, 'binary');
    });


    