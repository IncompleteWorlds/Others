import java.util.Scanner;


 interface RomanNumeralGenerator {
    public String generate(int number); 
 }

public class TestNumbers implements RomanNumeralGenerator {

    public static void main(String[] args) {
        // For testing purposes, it reads a number between 1 and 3999 from
        // the console and print the Roman number until user press Ctrl-C
        // It could be automated by reading several numbers from a file but
        // I have not had time
        // Numbers can be checked against a web page like wikipedia

        int number = 0;
        do {
            do {
                System.out.println("Enter a number between 1 and 3999: ");
                Scanner sc = new Scanner(System.in);
                number = sc.nextInt();
    
                if (number < 1 || number > 3999) {
                    System.out.println("ERROR: Incorrect number");
                } else {
                    break;
                }
            } while (true);
    
            System.out.println("Number: " + Integer.toString(number));
            TestNumbers t = new TestNumbers();
            System.out.println("Roman Number: " + t.generate(number));
            
        } while(true);
    }

    @Override
    public String generate(int inValue) {
        String output = "";

        if (inValue < 1 || inValue > 3999) {
            return output;
        }
        
        int mils = 0;
        int cents = 0;
        int decs = 0;
        int tmpValue = inValue;
        
        // Calculate the thousands part of the number 
        if (tmpValue >= 1000) {
            mils = inValue / 1000;
            tmpValue = inValue % 1000;   
            // Add to the output the Roman representation of thousands
            output = getMillards(mils);
        } 
        
        // Calculate the hundreds part of the number
        if (tmpValue >= 100){
            cents = tmpValue / 100;
            tmpValue = tmpValue % 100;
            // Add to the output the Roman representation of hundreds
            output += getCents(cents);
        } 
        
        // Calculate the tens part of the number
        if (tmpValue >= 10) {
            decs =  tmpValue / 10;
            tmpValue = tmpValue % 10;
            // Add to the output the Roman representation of tens
            output += getDecs(decs);
        } 
        
        // Finally, calculate the units
        if (tmpValue > 0) {
            // Add to the output the Roman representation of units
            output += getUnits(tmpValue);
        }

        return output;
    }

    private String getUnits(int rest) {
        String output = "";
        switch(rest) {
            case 0: output = "nulla"; break;
            case 1: output = "I"; break;
            case 2: output = "II"; break;
            case 3: output = "III"; break;
            case 4: output = "IV"; break;
            case 5: output = "V"; break;
            case 6: output = "VI"; break;
            case 7: output = "VII"; break;
            case 8: output = "VIII"; break;
            case 9: output = "IX"; break;
        }
        return output;
    }

    private String getDecs(int decs) {
        String output = "";
        switch(decs) {
            case 0: output = "nulla"; break;
            case 1: output = "X"; break;
            case 2: output = "XX"; break;
            case 3: output = "XXX"; break;
            case 4: output = "XL"; break;
            case 5: output = "L"; break;
            case 6: output = "LX"; break;
            case 7: output = "LXX"; break;
            case 8: output = "LXXX"; break;
            case 9: output = "XC"; break;
        }
        return output;
    }

    private String getCents(int cents) {
        String output = "";
        switch(cents) {
            case 0: output = "nulla"; break;
            case 1: output = "C"; break;
            case 2: output = "CC"; break;
            case 3: output = "CCC"; break;
            case 4: output = "CD"; break;
            case 5: output = "D"; break;
            case 6: output = "DC"; break;
            case 7: output = "DCC"; break;
            case 8: output = "DCCC"; break;
            case 9: output = "CM"; break;
        }
        return output;
    }

    private String getMillards(int mils) {
        String output = "";
        switch(mils) {
            case 0: output = "nulla"; break;
            case 1: output = "M"; break;
            case 2: output = "MM"; break;
            case 3: output = "MMM"; break;
            case 4: output = ""; break;
            case 5: output = ""; break;
            case 6: output = ""; break;
            case 7: output = ""; break;
            case 8: output = ""; break;
            case 9: output = ""; break;
        }
        return output;
    }
}
